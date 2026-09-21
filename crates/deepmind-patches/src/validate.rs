//! Every rule the repository enforces, as findings that name the file and say
//! what to do.
//!
//! Rules are numbered so a contributor can find them in `CONTRIBUTING.md`.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::{Path, PathBuf};

use deepmind_midi::ids::{Model, Slot as MidiSlot};
use deepmind_midi::program::ProgramName;

use crate::category::Category;
use crate::library::{Scan, Slot};
use crate::meta::{MAX_ABOUT_CHARS, MAX_DEMOS, MAX_VARIANT_CHARS, PatchMeta};
use crate::mp3::{Mp3Info, limits};
use crate::slug;
use crate::taxonomy::Axis;
use crate::{DEMOS_DIR, PRESETS_DIR, RESOURCES_DIR};

/// How bad a finding is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Severity {
    /// Worth fixing; fails only under `--strict`.
    Warning,
    /// Fails the build.
    Error,
}

/// One thing wrong, and what to do about it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    /// Error or warning.
    pub severity: Severity,
    /// The file, relative to the root.
    pub path: PathBuf,
    /// The line, when one applies.
    pub line: Option<u32>,
    /// What is wrong.
    pub message: String,
    /// What to do.
    pub hint: Option<String>,
}

impl Finding {
    /// An error with no hint.
    pub fn error(path: impl Into<PathBuf>, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            path: path.into(),
            line: None,
            message: message.into(),
            hint: None,
        }
    }

    /// A warning with no hint.
    pub fn warning(path: impl Into<PathBuf>, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Warning,
            ..Self::error(path, message)
        }
    }

    /// Adds the fix.
    #[must_use]
    pub fn hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    /// Adds the line.
    #[must_use]
    pub fn at(mut self, line: Option<u32>) -> Self {
        self.line = line;
        self
    }

    /// A GitHub Actions workflow command, which turns into an annotation on the
    /// pull request.
    #[must_use]
    pub fn annotation(&self) -> String {
        let kind = match self.severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
        };
        let line = self
            .line
            .map(|line| format!(",line={line}"))
            .unwrap_or_default();
        let hint = self
            .hint
            .as_deref()
            .map(|hint| format!(" {hint}"))
            .unwrap_or_default();
        format!(
            "::{kind} file={}{line}::{}{hint}",
            self.path.display(),
            self.message.replace('\n', "%0A")
        )
    }
}

impl fmt::Display for Finding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind = match self.severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
        };
        write!(f, "{kind}: {}", self.path.display())?;
        if let Some(line) = self.line {
            write!(f, ":{line}")?;
        }
        write!(f, ": {}", self.message)?;
        if let Some(hint) = &self.hint {
            write!(f, " {hint}")?;
        }
        Ok(())
    }
}

/// Runs every rule over a scan.
#[must_use]
pub fn run(scan: &Scan) -> Vec<Finding> {
    let mut findings = scan.problems.clone();
    check_taxonomy(scan, &mut findings);
    check_icons(scan, &mut findings);
    check_strays(scan, &mut findings);
    let mut ids: BTreeMap<String, PathBuf> = BTreeMap::new();
    let mut declared_demos: BTreeSet<PathBuf> = BTreeSet::new();
    for slot in &scan.slots {
        check_slot(scan, slot, &mut findings, &mut ids, &mut declared_demos);
    }
    check_demo_files(scan, &declared_demos, &mut findings);
    findings.sort_by(|a, b| (&a.path, a.line, &a.message).cmp(&(&b.path, b.line, &b.message)));
    findings.dedup();
    findings
}

/// Whether a run's findings should fail: any error, or any finding under
/// `strict`.
#[must_use]
pub fn fails(findings: &[Finding], strict: bool) -> bool {
    findings
        .iter()
        .any(|finding| strict || finding.severity == Severity::Error)
}

fn check_taxonomy(scan: &Scan, findings: &mut Vec<Finding>) {
    let Some(taxonomy) = &scan.taxonomy else {
        return;
    };
    for (axis, term) in taxonomy.malformed_terms() {
        findings.push(
            Finding::error(
                crate::TAXONOMY_FILE,
                format!("[{axis}] term {term:?} is not a lowercase word"),
            )
            .hint("Use lowercase ASCII letters, digits and hyphens."),
        );
    }
    for axis in Axis::ALL {
        for (term, about) in taxonomy.axis(axis) {
            if about.trim().is_empty() {
                findings.push(Finding::error(
                    crate::TAXONOMY_FILE,
                    format!("[{axis}] {term} has no description"),
                ));
            }
        }
    }
}

fn check_icons(scan: &Scan, findings: &mut Vec<Finding>) {
    let dir = Path::new(RESOURCES_DIR).join("icons").join("categories");
    for category in Category::ALL {
        match scan.icons.category(category) {
            None => findings.push(
                Finding::error(
                    dir.join(format!("{}.toml", category.name())),
                    "category icon is missing",
                )
                .hint("Add a file with a 7x7 `icon` and a one-line `about`."),
            ),
            Some(icon) if icon.is_blank() => findings.push(Finding::error(
                dir.join(format!("{}.toml", category.name())),
                "category icon is blank",
            )),
            Some(_) => {}
        }
    }
    for name in scan.icons.categories.keys() {
        if Category::from_name(name).is_none() {
            findings.push(
                Finding::error(dir.join(format!("{name}.toml")), "not a category name").hint(
                    "Category icons are named exactly as the instrument writes the category.",
                ),
            );
        }
    }
    for (name, icon) in &scan.icons.badges {
        if icon.is_blank() {
            findings.push(Finding::error(
                Path::new(RESOURCES_DIR)
                    .join("icons")
                    .join("badges")
                    .join(format!("{name}.toml")),
                "badge icon is blank",
            ));
        }
    }
}

fn check_strays(scan: &Scan, findings: &mut Vec<Finding>) {
    for path in &scan.stray {
        let hint = if path.starts_with(PRESETS_DIR) {
            "Only .syx and .toml files belong under presets/."
        } else if path.starts_with(RESOURCES_DIR) {
            "Only icons and palette.toml belong under resources/."
        } else {
            "Nothing else belongs at the root."
        };
        findings.push(Finding::error(path.clone(), "file does not belong here").hint(hint));
    }
}

fn check_slot(
    scan: &Scan,
    slot: &Slot,
    findings: &mut Vec<Finding>,
    ids: &mut BTreeMap<String, PathBuf>,
    declared_demos: &mut BTreeSet<PathBuf>,
) {
    let any_path = slot
        .syx
        .clone()
        .or_else(|| slot.toml.clone())
        .unwrap_or_else(|| slot.folder.join(&slot.stem));

    // Rule 1: both files.
    if slot.syx.is_none() {
        findings.push(
            Finding::error(any_path.clone(), "has no .syx beside it").hint(
                "Save the program from the instrument and put the .syx here with the same name.",
            ),
        );
    }
    if slot.toml.is_none() {
        findings.push(
            Finding::error(any_path.clone(), "has no .toml beside it")
                .hint("Write one; docs/format.md lists the fields."),
        );
    }

    // Rule 10: depth and folder names.
    let category = Category::from_name(&slot.category_dir);
    if category.is_none() {
        findings.push(
            Finding::error(any_path.clone(), format!("folder {:?} is not a category", slot.category_dir))
                .hint("The folders under presets/ are the instrument's twelve categories, spelled its way."),
        );
    }
    if slot.extra_depth > 0 {
        findings
            .push(Finding::error(any_path.clone(), "nested too deep").hint(
                "A patch sits in its category folder or in one collection folder inside it.",
            ));
    }
    if let Some(collection) = &slot.collection {
        if slug::slug(collection) != *collection {
            findings.push(
                Finding::error(
                    any_path.clone(),
                    format!("collection folder {collection:?} is not a clean name"),
                )
                .hint("No leading or trailing spaces, no / \\ : * ? \" < > | characters."),
            );
        }
    }

    check_program(slot, category, findings);
    let Some(meta) = &slot.meta else {
        return;
    };
    let toml_path = slot.toml.clone().unwrap_or(any_path);
    let text = slot.toml_text.as_deref().unwrap_or_default();
    check_meta(scan, slot, meta, &toml_path, text, findings);

    // Rule 8: stem.
    let expected = meta.stem();
    if expected != slot.stem {
        findings.push(
            Finding::error(
                toml_path.clone(),
                format!(
                    "file is named {:?} but name and author give {expected:?}",
                    slot.stem
                ),
            )
            .hint("Rename both files to that stem, or fix the fields."),
        );
    }

    // Rule 9: unique id.
    if let Some(category) = category {
        let id = slug::id(
            category.name(),
            slot.collection.as_deref(),
            &meta.name,
            &meta.author,
        );
        if let Some(other) = ids.insert(id.clone(), toml_path.clone()) {
            findings.push(
                Finding::error(
                    toml_path.clone(),
                    format!("id {id:?} is also used by {}", other.display()),
                )
                .hint("Two patches cannot share a name and an author in one folder."),
            );
        }
    }

    // Rule 12: demos.
    check_demos(scan, slot, meta, &toml_path, text, findings, declared_demos);
}

fn check_program(slot: &Slot, category: Option<Category>, findings: &mut Vec<Finding>) {
    let Some(syx_path) = &slot.syx else {
        return;
    };
    if slot.bytes.is_none() {
        return;
    }
    // Rule 4: exactly one program.
    let Some((program_slot, program)) = slot.programs.first() else {
        findings.push(
            Finding::error(syx_path.clone(), "holds no program dump")
                .hint("Save one program from the instrument, not the globals or a pattern."),
        );
        return;
    };
    if slot.programs.len() != 1 {
        findings.push(
            Finding::error(
                syx_path.clone(),
                format!("holds {} programs, expected one", slot.programs.len()),
            )
            .hint("One sound per file. Export a single program, not a bank."),
        );
        return;
    }
    // Rule 5: bank A, program 1.
    match program_slot {
        Some(stored) if *stored == MidiSlot::FIRST => {}
        Some(stored) => findings.push(
            Finding::error(syx_path.clone(), format!("is stored as {stored}, expected A1"))
                .hint("Run `validate normalise` or re-save the program to A1. The slot means nothing here."),
        ),
        None => findings.push(
            Finding::error(syx_path.clone(), "is an edit buffer dump, not a stored program")
                .hint("Save the program to a slot on the instrument and dump that."),
        ),
    }
    // Rule 6: category byte matches the folder.
    match (Category::of(program), category) {
        (None, _) => findings.push(
            Finding::error(
                syx_path.clone(),
                format!(
                    "category is {:?}, which is not one of the twelve",
                    Category::label_of_value(program.get(deepmind_midi::ParamId::ProgramCategory))
                ),
            )
            .hint("Set a category on the instrument first."),
        ),
        (Some(stored), Some(folder)) if stored != folder => findings.push(
            Finding::error(
                syx_path.clone(),
                format!("program says {stored} but sits in {folder}/"),
            )
            .hint("Move the files or change the category on the instrument."),
        ),
        _ => {}
    }
    // Rule 7: stored name equals the TOML name.
    if let Some(meta) = &slot.meta {
        let stored = program.name();
        if stored.as_str() != meta.name {
            findings.push(
                Finding::error(
                    slot.toml.clone().unwrap_or_else(|| syx_path.clone()),
                    format!(
                        "name is {:?} but the program is called {:?}",
                        meta.name,
                        stored.as_str()
                    ),
                )
                .hint("The TOML name must equal the sixteen characters stored in the program."),
            );
        }
    }
    let invalid: Vec<String> = program
        .invalid()
        .map(|(parameter, value)| format!("{} = {value}", parameter.name()))
        .collect();
    if !invalid.is_empty() {
        findings.push(
            Finding::warning(
                syx_path.clone(),
                format!("parameters out of range: {}", invalid.join(", ")),
            )
            .hint("The instrument may ignore or clamp these."),
        );
    }
}

fn check_meta(
    scan: &Scan,
    slot: &Slot,
    meta: &PatchMeta,
    path: &Path,
    text: &str,
    findings: &mut Vec<Finding>,
) {
    let line = |key: &str| line_of(text, key);
    // Rule 2: required fields.
    if meta.name.trim().is_empty() {
        findings.push(Finding::error(path, "name is empty").at(line("name")));
    } else if let Err(error) = ProgramName::new(&meta.name) {
        findings.push(
            Finding::error(path, format!("name cannot be a program name: {error}"))
                .at(line("name"))
                .hint("Sixteen printable ASCII characters at most."),
        );
    }
    if meta.author.trim().is_empty() {
        findings.push(Finding::error(path, "author is empty").at(line("author")));
    } else if meta.author.contains('@') {
        findings.push(
            Finding::error(path, "author looks like an email address")
                .at(line("author"))
                .hint("Use a name or a handle."),
        );
    }
    if meta.version == 0 {
        findings.push(
            Finding::error(path, "version must be 1 or more")
                .at(line("version"))
                .hint("Start at 1. Bump it whenever the .syx bytes change."),
        );
    }
    if meta.about.trim().is_empty() {
        findings.push(
            Finding::error(path, "about is empty")
                .at(line("about"))
                .hint("One or two sentences: what it sounds like, how to play it."),
        );
    } else if meta.about.chars().count() > MAX_ABOUT_CHARS {
        findings.push(
            Finding::error(path, format!("about is over {MAX_ABOUT_CHARS} characters"))
                .at(line("about")),
        );
    }
    // Rule 11: licence.
    if !meta.licence_is_valid() {
        findings.push(
            Finding::error(path, format!("licence {:?} is not an SPDX identifier", meta.licence))
                .at(line("licence"))
                .hint("Use an identifier from spdx.org/licenses, such as CC-BY-4.0, or exactly \"All rights reserved\"."),
        );
    }
    // Rule 2 and 3: vocabulary.
    if !meta.has_terms() {
        findings.push(
            Finding::error(path, "no genre, mood, timbre or role")
                .hint("Set at least one term from taxonomy.toml on any axis."),
        );
    }
    if let Some(taxonomy) = &scan.taxonomy {
        for axis in Axis::ALL {
            let mut seen = BTreeSet::new();
            for term in meta.terms(axis) {
                if !taxonomy.contains(axis, term) {
                    findings.push(
                        Finding::error(path, format!("{axis} term {term:?} is not in taxonomy.toml"))
                            .at(line(axis.name()))
                            .hint("Pick an existing term, or propose the new one in its own pull request."),
                    );
                }
                if !seen.insert(term) {
                    findings.push(
                        Finding::error(path, format!("{axis} lists {term:?} twice"))
                            .at(line(axis.name())),
                    );
                }
            }
        }
    }
    // Optional fields, checked when present.
    if let Some(model) = &meta.model {
        if !Model::ALL.iter().any(|known| known.name() == model) {
            let names: Vec<&str> = Model::ALL.iter().map(|known| known.name()).collect();
            findings.push(
                Finding::error(
                    path,
                    format!("model {model:?} is not one of {}", names.join(", ")),
                )
                .at(line("model")),
            );
        }
    }
    if let Some(firmware) = &meta.firmware {
        let ok = firmware
            .split('.')
            .all(|part| !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_digit()))
            && firmware.contains('.');
        if !ok {
            findings.push(
                Finding::error(
                    path,
                    format!("firmware {firmware:?} is not a version like 1.1.5"),
                )
                .at(line("firmware")),
            );
        }
    }
    for tag in &meta.tags {
        if tag.trim().is_empty() || tag.chars().count() > 32 {
            findings.push(
                Finding::error(path, format!("tag {tag:?} is empty or over 32 characters"))
                    .at(line("tags")),
            );
        }
    }
    if let Some(icon) = &meta.icon {
        if icon.is_blank() {
            findings.push(
                Finding::error(path, "icon is blank")
                    .at(line("icon"))
                    .hint("Draw something, or remove the field to use the category icon."),
            );
        }
    }
    for key in meta.unknown.keys() {
        findings.push(
            Finding::error(path, format!("unknown field {key:?}"))
                .at(line(key))
                .hint("docs/format.md lists every field. Anything the .syx already says is derived, not typed."),
        );
    }
    if meta.model.is_none() {
        findings.push(Finding::warning(path, "model is not set").hint(
            "A unison patch behaves differently on a DeepMind 6. Say what it was voiced on.",
        ));
    }
    let _ = slot;
}

fn check_demos(
    scan: &Scan,
    slot: &Slot,
    meta: &PatchMeta,
    path: &Path,
    text: &str,
    findings: &mut Vec<Finding>,
    declared: &mut BTreeSet<PathBuf>,
) {
    let line = line_of(text, "[[demo]]");
    if meta.demos.len() > MAX_DEMOS {
        findings.push(
            Finding::error(
                path,
                format!("{} demos, at most {MAX_DEMOS} allowed", meta.demos.len()),
            )
            .at(line),
        );
    }
    let mut variants = BTreeSet::new();
    let mut defaults = 0;
    for demo in &meta.demos {
        match &demo.variant {
            None => defaults += 1,
            Some(variant) => {
                if variant.chars().count() > MAX_VARIANT_CHARS
                    || slug::slug(variant) != *variant
                    || variant.contains(['(', ')'])
                {
                    findings.push(
                        Finding::error(
                            path,
                            format!("demo variant {variant:?} is not a clean label"),
                        )
                        .at(line)
                        .hint(
                            "At most 24 characters, no parentheses, no leading or trailing spaces.",
                        ),
                    );
                }
                if !variants.insert(variant) {
                    findings.push(
                        Finding::error(path, format!("demo variant {variant:?} is listed twice"))
                            .at(line),
                    );
                }
            }
        }
        let mut demo_path = PathBuf::from(DEMOS_DIR);
        demo_path.push(&slot.category_dir);
        if let Some(collection) = &slot.collection {
            demo_path.push(collection);
        }
        demo_path.push(meta.demo_file_name(demo));
        declared.insert(demo_path.clone());
        let absolute = scan.root.join(&demo_path);
        if !absolute.is_file() {
            findings.push(
                Finding::error(path, format!("demo {} is missing", demo_path.display()))
                    .at(line)
                    .hint("Put the MP3 there, or remove the [[demo]] entry."),
            );
            continue;
        }
        check_mp3(&absolute, &demo_path, findings);
    }
    if defaults > 1 {
        findings.push(
            Finding::error(path, "more than one demo without a variant")
                .at(line)
                .hint("One demo is the default; give the others a variant."),
        );
    }
}

fn check_mp3(absolute: &Path, relative: &Path, findings: &mut Vec<Finding>) {
    let size = std::fs::metadata(absolute).map_or(0, |meta| meta.len());
    if size > limits::MAX_BYTES {
        findings.push(
            Finding::error(
                relative,
                format!(
                    "is {} KB, over the {} KB cap",
                    size / 1024,
                    limits::MAX_BYTES / 1024
                ),
            )
            .hint("Trim it to 20 seconds at 128 kbit/s."),
        );
    }
    let info = match Mp3Info::read(absolute) {
        Ok(info) => info,
        Err(error) => {
            findings.push(Finding::error(relative, error.to_string()).hint("Demos are MP3 files."));
            return;
        }
    };
    if info.seconds > limits::MAX_SECONDS {
        findings.push(Finding::error(
            relative,
            format!(
                "runs {:.1} s, over the {} s limit",
                info.seconds,
                limits::MAX_SECONDS
            ),
        ));
    }
    if info.bitrate_kbps != limits::BITRATE_KBPS || !info.constant_bitrate {
        findings.push(Finding::error(
            relative,
            format!(
                "is {} kbit/s{}, expected {} kbit/s constant",
                info.bitrate_kbps,
                if info.constant_bitrate {
                    ""
                } else {
                    " variable"
                },
                limits::BITRATE_KBPS
            ),
        ));
    }
    if info.sample_rate != limits::SAMPLE_RATE {
        findings.push(Finding::error(
            relative,
            format!(
                "is {} Hz, expected {} Hz",
                info.sample_rate,
                limits::SAMPLE_RATE
            ),
        ));
    }
    if info.channels != 2 {
        findings.push(
            Finding::error(relative, "is mono, expected stereo")
                .hint("The chorus is part of the sound."),
        );
    }
}

fn check_demo_files(scan: &Scan, declared: &BTreeSet<PathBuf>, findings: &mut Vec<Finding>) {
    for file in &scan.demo_files {
        if file == Path::new(DEMOS_DIR).join("README.md").as_path() {
            continue;
        }
        if !declared.contains(file) {
            findings.push(
                Finding::error(file.clone(), "no patch declares this demo")
                    .hint("Add a [[demo]] entry to the patch's .toml, or delete the file."),
            );
        }
    }
}

/// The 1-based line a top-level key or table header sits on.
#[must_use]
pub fn line_of(text: &str, key: &str) -> Option<u32> {
    text.lines()
        .position(|line| {
            let line = line.trim_start();
            line == key
                || line
                    .strip_prefix(key)
                    .is_some_and(|rest| rest.trim_start().starts_with('='))
        })
        .and_then(|index| u32::try_from(index + 1).ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_lines() {
        let text = "name = \"x\"\nauthor=\"y\"\n\n[[demo]]\nabout = \"z\"\n";
        assert_eq!(line_of(text, "name"), Some(1));
        assert_eq!(line_of(text, "author"), Some(2));
        assert_eq!(line_of(text, "[[demo]]"), Some(4));
        assert_eq!(line_of(text, "about"), Some(5));
        assert_eq!(line_of(text, "nope"), None);
    }

    #[test]
    fn annotation_format() {
        let finding = Finding::error("presets/Bass/x.toml", "bad")
            .at(Some(3))
            .hint("Fix it.");
        assert_eq!(
            finding.annotation(),
            "::error file=presets/Bass/x.toml,line=3::bad Fix it."
        );
        assert_eq!(
            finding.to_string(),
            "error: presets/Bass/x.toml:3: bad Fix it."
        );
    }
}
