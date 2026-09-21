//! CI and contributor tool for the repository.
//!
//! Every command runs from the repository root. CI runs `check`, `versioning`,
//! `icons --check` and `crate-version`; the release workflow runs
//! `next-version`, `index` and `notes`; contributors use `new`, `normalise`
//! and `check`.

#![cfg_attr(test, allow(clippy::unwrap_used))]

mod git;

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use deepmind_midi::ParamId;
use deepmind_midi::ids::{Bank, DeviceId, ProgramNumber, ProtocolVersion};
use deepmind_midi::program::{Program, ProgramName};
use deepmind_midi::syx::{self, Writer};
use deepmind_patches::index::{HistoryEntry, Index};
use deepmind_patches::library::Scan;
use deepmind_patches::version::{Bump, Change, LibraryVersion, classify};
use deepmind_patches::{Category, Fingerprint, Library, PatchMeta, Severity, validate};

#[derive(Parser)]
#[command(
    name = "validate",
    about = "Checks and builds the deepmind-patches repository."
)]
struct Cli {
    /// Repository root.
    #[arg(long, global = true, default_value = ".")]
    root: PathBuf,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Runs every rule. Exit 1 on errors, or on warnings too under --strict.
    Check {
        /// Fail on warnings as well.
        #[arg(long)]
        strict: bool,
        /// Print GitHub Actions annotations instead of plain lines.
        #[arg(long)]
        github: bool,
    },
    /// Checks every version against a base ref: each changed patch bumped
    /// its own version, and the version in crates/deepmind-patches/Cargo.toml
    /// moved by at least what the change needs. Exit 1 when either did not.
    Versioning {
        /// The ref to compare with, usually origin/main.
        #[arg(long)]
        base: String,
    },
    /// Prints the version in crates/deepmind-patches/Cargo.toml, which is the
    /// version of everything.
    Version,
    /// Builds index.toml from a valid checkout.
    Index {
        /// Where to write.
        #[arg(long, default_value = "index.toml")]
        out: PathBuf,
        /// The library version this index describes.
        #[arg(long)]
        version: LibraryVersion,
        /// The commit the files are pinned to.
        #[arg(long)]
        commit: String,
    },
    /// Prints release notes for everything changed since a tag.
    Notes {
        /// The previous release tag, or nothing for the first release.
        #[arg(long)]
        since: Option<String>,
        /// The version being released.
        #[arg(long)]
        version: LibraryVersion,
    },
    /// Prints a one-paragraph summary of the checkout.
    Summary,
    /// Prints a pull request summary of the patches a change touches.
    PrSummary {
        /// The ref to compare with.
        #[arg(long)]
        base: String,
    },
    /// Creates a patch: the .syx, stored as A1, and a .toml to fill in.
    New {
        /// The category folder.
        #[arg(long)]
        category: Category,
        /// The program name, sixteen characters at most.
        #[arg(long)]
        name: String,
        /// Your name or handle.
        #[arg(long)]
        author: String,
        /// A .syx to take the program from. Without it, a blank program.
        #[arg(long)]
        from: Option<PathBuf>,
        /// A collection folder inside the category.
        #[arg(long)]
        collection: Option<String>,
        /// The licence to write.
        #[arg(long, default_value = "CC-BY-4.0")]
        licence: String,
    },
    /// Rewrites every .syx so it is stored as bank A, program 1.
    Normalise,
    /// Prints every icon as text, or checks that a written preview is current.
    Icons {
        /// Write the preview here instead of printing it.
        #[arg(long)]
        write: Option<PathBuf>,
        /// Exit 1 if the file at --write differs from a fresh render.
        #[arg(long)]
        check: bool,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli) {
        Ok(code) => code,
        Err(message) => {
            eprintln!("error: {message}");
            ExitCode::FAILURE
        }
    }
}

fn run(cli: Cli) -> Result<ExitCode, String> {
    let root = cli
        .root
        .canonicalize()
        .map_err(|error| format!("{}: {error}", cli.root.display()))?;
    match cli.command {
        Command::Check { strict, github } => check(&root, strict, github),
        Command::Versioning { base } => versioning(&root, &base),
        Command::Version => {
            println!("{}", current_version(&root)?);
            Ok(ExitCode::SUCCESS)
        }
        Command::Index {
            out,
            version,
            commit,
        } => {
            let library = open(&root)?;
            let history = history(&root, &library)?;
            let built = now();
            let index = Index::build(&library, version, &commit, &built, &history);
            std::fs::write(&out, index.to_toml().map_err(|error| error.to_string())?)
                .map_err(|error| format!("{}: {error}", out.display()))?;
            eprintln!(
                "wrote {} with {} patches",
                out.display(),
                index.patches.len()
            );
            Ok(ExitCode::SUCCESS)
        }
        Command::Notes { since, version } => {
            print!("{}", notes(&root, since.as_deref(), version)?);
            Ok(ExitCode::SUCCESS)
        }
        Command::Summary => {
            println!("{}", summary(&open(&root)?));
            Ok(ExitCode::SUCCESS)
        }
        Command::PrSummary { base } => {
            print!("{}", pr_summary(&root, &base)?);
            Ok(ExitCode::SUCCESS)
        }
        Command::New {
            category,
            name,
            author,
            from,
            collection,
            licence,
        } => new_patch(
            &root,
            category,
            &name,
            &author,
            from.as_deref(),
            collection.as_deref(),
            &licence,
        ),
        Command::Normalise => normalise(&root),
        Command::Icons { write, check } => icons(&root, write.as_deref(), check),
    }
}

fn open(root: &Path) -> Result<Library, String> {
    let scan = Library::scan(root).map_err(|error| error.to_string())?;
    let findings = validate::run(&scan);
    if validate::fails(&findings, false) {
        for finding in &findings {
            eprintln!("{finding}");
        }
        return Err("the checkout does not validate; fix the errors above first".to_owned());
    }
    Ok(Library::from_scan(scan))
}

fn check(root: &Path, strict: bool, github: bool) -> Result<ExitCode, String> {
    let scan = Library::scan(root).map_err(|error| error.to_string())?;
    let findings = validate::run(&scan);
    for finding in &findings {
        if github {
            println!("{}", finding.annotation());
        } else {
            println!("{finding}");
        }
    }
    let errors = findings
        .iter()
        .filter(|f| f.severity == Severity::Error)
        .count();
    let warnings = findings.len() - errors;
    let patches = scan
        .slots
        .iter()
        .filter(|slot| slot.meta.is_some() && slot.program().is_some())
        .count();
    eprintln!("{patches} patches, {errors} errors, {warnings} warnings");
    Ok(if validate::fails(&findings, strict) {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    })
}

/// One patch's state on one side of a diff.
struct Side {
    meta: PatchMeta,
    fingerprint: Option<Fingerprint>,
}

fn side_at(
    root: &Path,
    rev: Option<&str>,
    toml_path: &Path,
    syx_path: &Path,
) -> Result<Option<Side>, String> {
    let toml_text = match rev {
        Some(rev) => git::show(root, rev, toml_path)?,
        None => std::fs::read(root.join(toml_path)).ok(),
    };
    let Some(toml_text) = toml_text else {
        return Ok(None);
    };
    let meta = PatchMeta::parse(&String::from_utf8_lossy(&toml_text))
        .map_err(|error| format!("{}: {error}", toml_path.display()))?;
    let syx_bytes = match rev {
        Some(rev) => git::show(root, rev, syx_path)?,
        None => std::fs::read(root.join(syx_path)).ok(),
    };
    let fingerprint = syx_bytes.and_then(|bytes| {
        let mut programs = syx::File::new(&bytes).programs();
        let entry = programs.next()?.ok()?;
        Some(Fingerprint::of(&entry.program))
    });
    Ok(Some(Side { meta, fingerprint }))
}

fn versioning(root: &Path, base: &str) -> Result<ExitCode, String> {
    let changes = git::changes(root, base)?;
    let mut failed = false;

    // Each changed patch bumped its own version if its bytes changed.
    let mut seen = std::collections::BTreeSet::new();
    for (change, path) in &changes {
        if !path.starts_with(deepmind_patches::PRESETS_DIR) || *change == Change::Deleted {
            continue;
        }
        let stem = path.with_extension("");
        if !seen.insert(stem.clone()) {
            continue;
        }
        let toml_path = stem.with_extension("toml");
        let syx_path = stem.with_extension("syx");
        let Some(head) = side_at(root, None, &toml_path, &syx_path)? else {
            continue;
        };
        let before = side_at(root, Some(base), &toml_path, &syx_path)?;
        match before {
            None => {
                if head.meta.version != 1 {
                    println!(
                        "::warning file={}::new patch starts at version {}, expected 1",
                        toml_path.display(),
                        head.meta.version
                    );
                }
            }
            Some(before) => {
                let bytes_changed = before.fingerprint != head.fingerprint;
                if bytes_changed && head.meta.version <= before.meta.version {
                    failed = true;
                    println!(
                        "::error file={}::the .syx changed but version is still {}. Set version = {}.",
                        toml_path.display(),
                        head.meta.version,
                        before.meta.version + 1
                    );
                } else if !bytes_changed && head.meta.version != before.meta.version {
                    println!(
                        "::warning file={}::version changed from {} to {} but the sound did not. Only bump when the .syx bytes change.",
                        toml_path.display(),
                        before.meta.version,
                        head.meta.version
                    );
                }
            }
        }
    }

    // The one version moved by at least what the change needs.
    let manifest_path = Path::new(CRATE_DIR).join("Cargo.toml");
    let file = manifest_path.display();
    let head = current_version(root)?;
    let before = git::show(root, base, &manifest_path)?
        .map(|text| manifest_version(&text))
        .transpose()?;
    let content = classify(
        changes
            .iter()
            .map(|(change, path)| (*change, path.as_path())),
    );
    // The version line lives in the crate, so a bump alone is not a code
    // change. Anything else under the crate, or any other edit to its
    // manifest, is.
    let manifest_changed_beyond_version = match &before {
        Some(_) => {
            let before_text = git::show(root, base, &manifest_path)?.unwrap_or_default();
            let head_text = std::fs::read(root.join(&manifest_path)).unwrap_or_default();
            without_version_line(&before_text) != without_version_line(&head_text)
        }
        None => true,
    };
    let code_changed = changes.iter().any(|(_, path)| {
        path.starts_with(CRATE_DIR) && (path != &manifest_path || manifest_changed_beyond_version)
    });
    let required = if code_changed {
        content.max(Bump::Patch)
    } else {
        content
    };
    let year = current_year();
    let step = match before {
        None => Bump::Release,
        Some(before) if before == head => {
            if required != Bump::None {
                failed = true;
                let mut expected = format!(
                    "{} for a fix or {} for an addition",
                    head.next(Bump::Patch, head.year),
                    head.next(Bump::Release, head.year),
                );
                if year > head.year {
                    expected = format!("{} to start the year", head.next(Bump::Year, year));
                }
                println!(
                    "::error file={file}::this change needs a {} bump but version is still {head}. Set version = {expected}, then run cargo check so Cargo.lock follows.",
                    bump_label(required)
                );
            }
            Bump::None
        }
        Some(before) => match before.step_to(head, year) {
            Ok(step) => {
                if year > before.year && step != Bump::Year {
                    failed = true;
                    println!(
                        "::error file={file}::the year moved on; the next version is {}, not {head}.",
                        before.next(Bump::Year, year)
                    );
                }
                if step < required {
                    failed = true;
                    println!(
                        "::error file={file}::this change needs a {} bump but {before} to {head} is a {} bump. Set version = {}.",
                        bump_label(required),
                        bump_label(step),
                        before.next(required, year)
                    );
                }
                if required == Bump::None {
                    println!(
                        "::warning file={file}::version went from {before} to {head} but nothing that ships changed. Merging still releases it."
                    );
                }
                step
            }
            Err(error) => {
                failed = true;
                println!("::error file={file}::{error}");
                Bump::None
            }
        },
    };
    let changelog_changed = changes
        .iter()
        .any(|(_, path)| path == Path::new("CHANGELOG.md"));
    if code_changed && !changelog_changed {
        println!(
            "::warning file=CHANGELOG.md::{CRATE_DIR} changed without a line in CHANGELOG.md."
        );
    }

    let label = bump_label(step);
    println!("version={head}");
    println!("bump={label}");
    println!("needs={}", bump_label(required));
    println!("code_changed={code_changed}");
    if let Ok(output) = std::env::var("GITHUB_OUTPUT") {
        use std::io::Write as _;
        if let Ok(mut file) = std::fs::OpenOptions::new().append(true).open(output) {
            let _ = writeln!(file, "version={head}");
            let _ = writeln!(file, "bump={label}");
            let _ = writeln!(file, "needs={}", bump_label(required));
            let _ = writeln!(file, "code_changed={code_changed}");
        }
    }
    Ok(if failed {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    })
}

fn without_version_line(manifest: &[u8]) -> String {
    String::from_utf8_lossy(manifest)
        .lines()
        .filter(|line| !line.trim_start().starts_with("version"))
        .collect::<Vec<_>>()
        .join("\n")
}

const fn bump_label(bump: Bump) -> &'static str {
    match bump {
        Bump::None => "none",
        Bump::Patch => "patch",
        Bump::Release => "release",
        Bump::Year => "year",
    }
}

/// Where the published crate lives, relative to the repository root. Its
/// Cargo.toml carries the version of everything.
const CRATE_DIR: &str = "crates/deepmind-patches";

fn manifest_version(text: &[u8]) -> Result<LibraryVersion, String> {
    let manifest: toml::Value = toml::from_str(&String::from_utf8_lossy(text))
        .map_err(|error| format!("{CRATE_DIR}/Cargo.toml: {error}"))?;
    manifest
        .get("package")
        .and_then(|package| package.get("version"))
        .and_then(toml::Value::as_str)
        .ok_or_else(|| format!("{CRATE_DIR}/Cargo.toml has no package.version"))?
        .parse()
        .map_err(|error| format!("{CRATE_DIR}/Cargo.toml: {error}"))
}

fn current_version(root: &Path) -> Result<LibraryVersion, String> {
    let manifest_path = Path::new(CRATE_DIR).join("Cargo.toml");
    manifest_version(
        &std::fs::read(root.join(&manifest_path))
            .map_err(|error| format!("{}: {error}", manifest_path.display()))?,
    )
}

fn history(root: &Path, library: &Library) -> Result<BTreeMap<String, Vec<HistoryEntry>>, String> {
    let mut out = BTreeMap::new();
    for patch in library.patches() {
        let mut by_version: BTreeMap<u32, Fingerprint> = BTreeMap::new();
        for commit in git::log(root, &patch.syx_path)? {
            let Some(side) = side_at(root, Some(&commit), &patch.toml_path, &patch.syx_path)?
            else {
                continue;
            };
            if let Some(fingerprint) = side.fingerprint {
                // Newest commit first, so the first fingerprint seen per
                // version is the one that version ended up as.
                by_version.entry(side.meta.version).or_insert(fingerprint);
            }
        }
        let entries: Vec<HistoryEntry> = by_version
            .into_iter()
            .map(|(version, fingerprint)| HistoryEntry {
                version,
                fingerprint,
            })
            .collect();
        out.insert(patch.id.clone(), entries);
    }
    Ok(out)
}

fn notes(root: &Path, since: Option<&str>, version: LibraryVersion) -> Result<String, String> {
    let library = open(root)?;
    let mut added = Vec::new();
    let mut updated = Vec::new();
    match since {
        None => added.extend(library.patches().iter().map(|patch| (patch, None::<u32>))),
        Some(since) => {
            let changes = git::changes(root, since)?;
            for patch in library.patches() {
                let touched = changes
                    .iter()
                    .any(|(_, path)| *path == patch.syx_path || *path == patch.toml_path);
                if !touched {
                    continue;
                }
                match side_at(root, Some(since), &patch.toml_path, &patch.syx_path)? {
                    None => added.push((patch, None)),
                    Some(before) => updated.push((patch, Some(before.meta.version))),
                }
            }
        }
    }
    let mut out = String::new();
    let _ = writeln!(out, "## {version}\n");
    let _ = writeln!(out, "{} patches in the library.\n", library.patches().len());
    if !added.is_empty() {
        let _ = writeln!(out, "### Added\n");
        for (patch, _) in &added {
            let _ = writeln!(
                out,
                "- **{}** by {} ({})",
                patch.meta.name, patch.meta.author, patch.category
            );
        }
        out.push('\n');
    }
    if !updated.is_empty() {
        let _ = writeln!(out, "### Updated\n");
        for (patch, before) in &updated {
            let change = match before {
                Some(before) if *before != patch.meta.version => {
                    format!("v{before} to v{}", patch.meta.version)
                }
                _ => "metadata".to_owned(),
            };
            let _ = writeln!(
                out,
                "- **{}** by {} ({}): {change}",
                patch.meta.name, patch.meta.author, patch.category
            );
        }
        out.push('\n');
    }
    if added.is_empty() && updated.is_empty() {
        out.push_str("No patch changes.\n\n");
    }
    out.push_str("Assets: `index.toml`, `patches.tar.gz` (presets/), `demos.tar.gz` (demos/).\n");
    Ok(out)
}

fn summary(library: &Library) -> String {
    let mut per_category: BTreeMap<&str, usize> = BTreeMap::new();
    for patch in library.patches() {
        *per_category.entry(patch.category.name()).or_default() += 1;
    }
    let authors: std::collections::BTreeSet<&str> = library
        .patches()
        .iter()
        .map(|p| p.meta.author.as_str())
        .collect();
    let demos: usize = library.patches().iter().map(|p| p.meta.demos.len()).sum();
    let categories: Vec<String> = Category::ALL
        .iter()
        .filter_map(|category| {
            per_category
                .get(category.name())
                .map(|count| format!("{category} {count}"))
        })
        .collect();
    format!(
        "{} patches by {} authors, {} demos. {}.",
        library.patches().len(),
        authors.len(),
        demos,
        categories.join(", ")
    )
}

fn pr_summary(root: &Path, base: &str) -> Result<String, String> {
    let scan = Library::scan(root).map_err(|error| error.to_string())?;
    let library = Library::from_scan(scan);
    let changes = git::changes(root, base)?;
    let mut out = String::new();
    let mut rows = 0;
    for patch in library.patches() {
        let touched = changes
            .iter()
            .any(|(_, path)| *path == patch.syx_path || *path == patch.toml_path);
        if !touched {
            continue;
        }
        if rows == 0 {
            out.push_str(
                "| Patch | Category | Version | Effects | Arp | Voices | Routings | Licence |\n",
            );
            out.push_str("| --- | --- | --- | --- | --- | --- | --- | --- |\n");
        }
        rows += 1;
        let d = &patch.derived;
        let _ = writeln!(
            out,
            "| {} by {} | {} | {} | {} | {} | {} | {} | {} |",
            patch.meta.name,
            patch.meta.author,
            patch.category,
            patch.meta.version,
            if d.effects.is_empty() {
                "bypassed".to_owned()
            } else {
                d.effects.join(", ")
            },
            d.arp_mode.as_deref().unwrap_or("off"),
            d.polyphony,
            d.routings,
            patch.meta.licence
        );
    }
    if rows == 0 {
        out.push_str("No patches changed.\n");
    } else {
        out.push_str("\nRead from the `.syx` bytes, not the description.\n");
    }
    Ok(out)
}

fn new_patch(
    root: &Path,
    category: Category,
    name: &str,
    author: &str,
    from: Option<&Path>,
    collection: Option<&str>,
    licence: &str,
) -> Result<ExitCode, String> {
    let program_name = ProgramName::new(name).map_err(|error| format!("name: {error}"))?;
    let mut program = match from {
        Some(from) => {
            let bytes =
                std::fs::read(from).map_err(|error| format!("{}: {error}", from.display()))?;
            let mut programs: Vec<Program> = syx::File::new(&bytes)
                .programs()
                .filter_map(Result::ok)
                .map(|entry| entry.program)
                .collect();
            if programs.len() != 1 {
                return Err(format!(
                    "{} holds {} programs, expected one",
                    from.display(),
                    programs.len()
                ));
            }
            programs.remove(0)
        }
        None => Program::new(ProtocolVersion::V7),
    };
    program.set_name(program_name);
    program
        .set(ParamId::ProgramCategory, category.value())
        .map_err(|error| error.to_string())?;
    let meta = PatchMeta {
        name: name.to_owned(),
        author: author.to_owned(),
        version: 1,
        about: String::new(),
        licence: licence.to_owned(),
        genre: Vec::new(),
        mood: Vec::new(),
        timbre: Vec::new(),
        role: Vec::new(),
        icon: None,
        created: today(),
        model: None,
        firmware: None,
        source: None,
        tags: Vec::new(),
        demos: Vec::new(),
        unknown: toml::Table::new(),
    };
    let mut folder = root
        .join(deepmind_patches::PRESETS_DIR)
        .join(category.name());
    if let Some(collection) = collection {
        folder.push(collection);
    }
    std::fs::create_dir_all(&folder).map_err(|error| format!("{}: {error}", folder.display()))?;
    let stem = meta.stem();
    let syx_path = folder.join(format!("{stem}.syx"));
    let toml_path = folder.join(format!("{stem}.toml"));
    if syx_path.exists() || toml_path.exists() {
        return Err(format!("{} already exists", syx_path.display()));
    }
    std::fs::write(&syx_path, program_bytes(&program)?)
        .map_err(|error| format!("{}: {error}", syx_path.display()))?;
    let mut text = meta.to_toml().map_err(|error| error.to_string())?;
    text = text.replace(
        "about = \"\"",
        "about = \"\" # What it sounds like, how to play it.",
    );
    text.push_str("\n# Pick at least one term from taxonomy.toml on any axis.\n# genre  = []\n# mood   = []\n# timbre = []\n# role   = []\n");
    std::fs::write(&toml_path, text)
        .map_err(|error| format!("{}: {error}", toml_path.display()))?;
    println!("{}\n{}", syx_path.display(), toml_path.display());
    Ok(ExitCode::SUCCESS)
}

fn program_bytes(program: &Program) -> Result<Vec<u8>, String> {
    let mut buffer = vec![0u8; syx::MAX_PROGRAM_FRAME_LEN];
    let mut writer = Writer::new(&mut buffer, DeviceId::Broadcast);
    writer
        .push_program(Bank::A, ProgramNumber::FIRST, program)
        .map_err(|error| error.to_string())?;
    let written = writer.finish();
    buffer.truncate(written);
    Ok(buffer)
}

fn normalise(root: &Path) -> Result<ExitCode, String> {
    let scan = Library::scan(root).map_err(|error| error.to_string())?;
    let mut rewritten = 0;
    for slot in &scan.slots {
        let (Some(path), [(stored, program)]) = (&slot.syx, slot.programs.as_slice()) else {
            continue;
        };
        if *stored == Some(deepmind_midi::ids::Slot::FIRST) {
            continue;
        }
        let bytes = program_bytes(program)?;
        std::fs::write(root.join(path), bytes)
            .map_err(|error| format!("{}: {error}", path.display()))?;
        println!("{}: rewritten as A1", path.display());
        rewritten += 1;
    }
    eprintln!("{rewritten} files rewritten");
    Ok(ExitCode::SUCCESS)
}

fn icons(root: &Path, write: Option<&Path>, check: bool) -> Result<ExitCode, String> {
    let scan = Library::scan(root).map_err(|error| error.to_string())?;
    let text = render_icons(&scan);
    match write {
        None => {
            print!("{text}");
            Ok(ExitCode::SUCCESS)
        }
        Some(path) if check => {
            let current = std::fs::read_to_string(root.join(path)).unwrap_or_default();
            if current == text {
                Ok(ExitCode::SUCCESS)
            } else {
                eprintln!(
                    "{} is stale. Run `cargo run -p validate -- icons --write {}`.",
                    path.display(),
                    path.display()
                );
                Ok(ExitCode::FAILURE)
            }
        }
        Some(path) => {
            std::fs::write(root.join(path), text)
                .map_err(|error| format!("{}: {error}", path.display()))?;
            Ok(ExitCode::SUCCESS)
        }
    }
}

fn render_icons(scan: &Scan) -> String {
    let mut out = String::from("# Icons\n\nGenerated from `resources/icons/`. Do not edit.\n");
    let mut section = |title: &str, icons: &BTreeMap<String, deepmind_patches::Icon>| {
        let _ = writeln!(out, "\n## {title}\n");
        let names: Vec<&String> = icons.keys().collect();
        for row in names.chunks(6) {
            out.push_str("```\n");
            for y in 0..deepmind_patches::Icon::SIDE {
                let line: Vec<String> = row
                    .iter()
                    .map(|name| {
                        (0..deepmind_patches::Icon::SIDE)
                            .map(|x| {
                                if icons[*name].is_lit(x, y) {
                                    "██"
                                } else {
                                    "··"
                                }
                            })
                            .collect()
                    })
                    .collect();
                let _ = writeln!(out, "{}", line.join("   "));
            }
            let labels: Vec<String> = row.iter().map(|name| format!("{name:<14}")).collect();
            let _ = writeln!(out, "{}", labels.join("   ").trim_end());
            out.push_str("```\n");
        }
    };
    section("Categories", &scan.icons.categories);
    section("Badges", &scan.icons.badges);
    section("Genre", &scan.icons.genre);
    section("Mood", &scan.icons.mood);
    section("Timbre", &scan.icons.timbre);
    section("Role", &scan.icons.role);
    out
}

/// Seconds since the Unix epoch, now.
fn unix_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_secs())
}

/// The proleptic Gregorian date of a Unix timestamp, in UTC.
///
/// Howard Hinnant's `civil_from_days`, which is what every date library
/// does underneath. It keeps the tool off a date crate for three lines of
/// arithmetic.
fn civil_date(unix: u64) -> (i64, u32, u32) {
    let days = i64::try_from(unix / 86_400).unwrap_or(i64::MAX / 4);
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    (year, month as u32, day as u32)
}

/// The current time as RFC 3339 in UTC, to the second: `2026-09-21T10:31:07Z`.
fn now() -> String {
    let unix = unix_now();
    let (year, month, day) = civil_date(unix);
    let seconds = unix % 86_400;
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z",
        seconds / 3600,
        seconds % 3600 / 60,
        seconds % 60
    )
}

fn today() -> Option<toml::value::Datetime> {
    let (year, month, day) = civil_date(unix_now());
    format!("{year:04}-{month:02}-{day:02}").parse().ok()
}

fn current_year() -> u32 {
    u32::try_from(civil_date(unix_now()).0 % 100).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::civil_date;

    #[test]
    fn civil_dates() {
        assert_eq!(civil_date(0), (1970, 1, 1));
        assert_eq!(civil_date(86_399), (1970, 1, 1));
        assert_eq!(civil_date(86_400), (1970, 1, 2));
        // 2000-02-29 and 2000-03-01, across a leap day in a leap century.
        assert_eq!(civil_date(951_782_400), (2000, 2, 29));
        assert_eq!(civil_date(951_868_800), (2000, 3, 1));
        // 2026-09-21T10:31:07Z.
        assert_eq!(civil_date(1_789_986_667), (2026, 9, 21));
        assert_eq!(civil_date(4_102_444_800), (2100, 1, 1));
    }
}
