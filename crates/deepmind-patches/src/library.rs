//! A checkout of the repository: every patch, demo and icon it holds.
//!
//! [`Library::scan`] reads everything and records what it could not read.
//! [`crate::validate`] turns a scan into findings. [`Library::open`] does both
//! and hands back typed patches only when nothing is wrong, which is what an
//! application wants; a validator wants the scan.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use deepmind_midi::program::Program;
use deepmind_midi::syx;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

use crate::category::Category;
use crate::derived::Derived;
use crate::error::{Error, Result};
use crate::fingerprint::{Fingerprint, sha256_hex};
use crate::icon::Icon;
use crate::meta::PatchMeta;
use crate::slug;
use crate::taxonomy::Taxonomy;
use crate::validate::{Finding, Severity};
use crate::{DEMOS_DIR, PRESETS_DIR, RESOURCES_DIR, TAXONOMY_FILE};

/// One icon file under `resources/icons/`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IconFile {
    /// The picture.
    pub icon: Icon,
    /// What it shows, one line.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
}

/// The icons a checkout carries: one per category, plus badges.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Icons {
    /// Keyed by category name. Complete in a valid checkout.
    #[serde(default)]
    pub categories: BTreeMap<String, Icon>,
    /// Keyed by file stem: `arp`, `seq`, `unison`, `fx`.
    #[serde(default)]
    pub badges: BTreeMap<String, Icon>,
}

impl Icons {
    /// The icon for a category, if present.
    #[must_use]
    pub fn category(&self, category: Category) -> Option<&Icon> {
        self.categories.get(category.name())
    }
}

/// `resources/palette.toml`: named colours, grouped. Carried, not interpreted.
pub type Palette = BTreeMap<String, BTreeMap<String, String>>;

/// One filename stem in one folder: the `.syx`, the `.toml`, and whatever
/// reading them produced. What the validator looks at.
#[derive(Debug, Clone)]
pub struct Slot {
    /// Folder relative to the root: `presets/Pad/Aurora Pads`.
    pub folder: PathBuf,
    /// The first folder under `presets/`, which should be a category name.
    pub category_dir: String,
    /// The second folder, if any.
    pub collection: Option<String>,
    /// Folders below the category beyond the one allowed collection.
    pub extra_depth: usize,
    /// The filename without extension.
    pub stem: String,
    /// The `.syx` path relative to the root, if present.
    pub syx: Option<PathBuf>,
    /// The `.toml` path relative to the root, if present.
    pub toml: Option<PathBuf>,
    /// The `.toml` text, if it could be read.
    pub toml_text: Option<String>,
    /// The parsed metadata, if it parsed.
    pub meta: Option<PatchMeta>,
    /// The `.syx` bytes, if they could be read.
    pub bytes: Option<Vec<u8>>,
    /// Every program the file carried, with the slot each names.
    pub programs: Vec<(Option<deepmind_midi::ids::Slot>, Program)>,
}

impl Slot {
    /// The one program, when the file holds exactly one.
    #[must_use]
    pub fn program(&self) -> Option<&Program> {
        match self.programs.as_slice() {
            [(_, program)] => Some(program),
            _ => None,
        }
    }
}

/// Everything read from a checkout, including what failed to read.
#[derive(Debug, Clone)]
pub struct Scan {
    /// The checkout root.
    pub root: PathBuf,
    /// The vocabularies, if the file parsed.
    pub taxonomy: Option<Taxonomy>,
    /// The icons that parsed.
    pub icons: Icons,
    /// The palette, if present and parsed.
    pub palette: Option<Palette>,
    /// Every stem under `presets/`, in path order.
    pub slots: Vec<Slot>,
    /// Every file under `demos/`, relative to the root.
    pub demo_files: Vec<PathBuf>,
    /// Every file under the root that no rule accounts for.
    pub stray: Vec<PathBuf>,
    /// Problems met while reading, before any rule ran.
    pub problems: Vec<Finding>,
}

/// One valid patch.
#[derive(Debug, Clone)]
pub struct Patch {
    /// `bass/acid-growl-nyx`.
    pub id: String,
    /// The folder, which the program byte agrees with.
    pub category: Category,
    /// The collection folder, if any.
    pub collection: Option<String>,
    /// The filename stem.
    pub stem: String,
    /// The `.syx` relative to the root.
    pub syx_path: PathBuf,
    /// The `.toml` relative to the root.
    pub toml_path: PathBuf,
    /// What the author wrote.
    pub meta: PatchMeta,
    /// The program.
    pub program: Program,
    /// What the program says.
    pub derived: Derived,
    /// Identity of the sound.
    pub fingerprint: Fingerprint,
    /// SHA-256 of the `.syx` file, hex.
    pub sha256: String,
    /// The `.syx` bytes.
    pub bytes: Vec<u8>,
}

impl Patch {
    /// The path a demo lives at, relative to the root.
    #[must_use]
    pub fn demo_path(&self, demo: &crate::meta::Demo) -> PathBuf {
        let mut path = PathBuf::from(DEMOS_DIR);
        path.push(self.category.name());
        if let Some(collection) = &self.collection {
            path.push(collection);
        }
        path.push(self.meta.demo_file_name(demo));
        path
    }
}

/// A valid checkout.
#[derive(Debug, Clone)]
pub struct Library {
    /// The checkout root.
    pub root: PathBuf,
    /// The vocabularies.
    pub taxonomy: Taxonomy,
    /// The icons.
    pub icons: Icons,
    /// The palette, if any.
    pub palette: Option<Palette>,
    patches: Vec<Patch>,
}

impl Library {
    /// Reads a checkout. Nothing is judged; see [`crate::validate`].
    pub fn scan(root: &Path) -> Result<Scan> {
        let root = root
            .canonicalize()
            .map_err(|error| Error::io(root, error))?;
        let mut scan = Scan {
            root: root.clone(),
            taxonomy: None,
            icons: Icons::default(),
            palette: None,
            slots: Vec::new(),
            demo_files: Vec::new(),
            stray: Vec::new(),
            problems: Vec::new(),
        };
        scan.read_taxonomy();
        scan.read_icons();
        scan.read_palette();
        scan.read_presets()?;
        scan.read_demos()?;
        scan.find_strays()?;
        Ok(scan)
    }

    /// Reads and validates a checkout, refusing one with errors.
    ///
    /// # Errors
    ///
    /// [`Error::Invalid`] with the count when validation fails; run
    /// [`crate::validate::run`] on a scan to see the findings.
    pub fn open(root: impl AsRef<Path>) -> Result<Self> {
        let scan = Self::scan(root.as_ref())?;
        let findings = crate::validate::run(&scan);
        let errors = findings
            .iter()
            .filter(|finding| finding.severity == Severity::Error)
            .count();
        if errors > 0 {
            return Err(Error::Invalid(errors));
        }
        Ok(Self::from_scan(scan))
    }

    /// Builds typed patches out of a scan, keeping every slot that is whole.
    ///
    /// A slot missing a file, a program or a parse is skipped. Call
    /// [`crate::validate::run`] first when that matters.
    #[must_use]
    pub fn from_scan(scan: Scan) -> Self {
        let mut patches = Vec::with_capacity(scan.slots.len());
        for slot in scan.slots {
            let program = slot.program().cloned();
            let (Some(meta), Some(program), Some(syx_path), Some(toml_path), Some(bytes)) =
                (slot.meta, program, slot.syx, slot.toml, slot.bytes)
            else {
                continue;
            };
            let Some(category) = Category::from_name(&slot.category_dir) else {
                continue;
            };
            let derived = Derived::of(&program);
            patches.push(Patch {
                id: slug::id(
                    category.name(),
                    slot.collection.as_deref(),
                    &meta.name,
                    &meta.author,
                ),
                category,
                collection: slot.collection,
                stem: slot.stem,
                syx_path,
                toml_path,
                fingerprint: Fingerprint::of(&program),
                sha256: sha256_hex(&bytes),
                meta,
                program,
                derived,
                bytes,
            });
        }
        patches.sort_by(|a, b| a.id.cmp(&b.id));
        Self {
            root: scan.root,
            taxonomy: scan.taxonomy.unwrap_or_default(),
            icons: scan.icons,
            palette: scan.palette,
            patches,
        }
    }

    /// Every patch, sorted by id.
    #[must_use]
    pub fn patches(&self) -> &[Patch] {
        &self.patches
    }

    /// One patch by id.
    #[must_use]
    pub fn patch(&self, id: &str) -> Option<&Patch> {
        self.patches.iter().find(|patch| patch.id == id)
    }

    /// The patches in one category.
    pub fn in_category(&self, category: Category) -> impl Iterator<Item = &Patch> {
        self.patches
            .iter()
            .filter(move |patch| patch.category == category)
    }

    /// The patch whose current bytes carry a fingerprint.
    #[must_use]
    pub fn by_fingerprint(&self, fingerprint: &Fingerprint) -> Option<&Patch> {
        self.patches
            .iter()
            .find(|patch| patch.fingerprint == *fingerprint)
    }

    /// The icon to show for a patch: its own, else its category's, else blank.
    #[must_use]
    pub fn icon_for(&self, patch: &Patch) -> Icon {
        patch
            .meta
            .icon
            .or_else(|| self.icons.category(patch.category).copied())
            .unwrap_or_default()
    }
}

impl Scan {
    fn problem(&mut self, path: impl Into<PathBuf>, message: impl Into<String>) {
        self.problems.push(Finding::error(path, message));
    }

    fn read_taxonomy(&mut self) {
        let path = self.root.join(TAXONOMY_FILE);
        match Taxonomy::read(&path) {
            Ok(taxonomy) => self.taxonomy = Some(taxonomy),
            Err(error) => self.problem(TAXONOMY_FILE, error.to_string()),
        }
    }

    fn read_icons(&mut self) {
        for group in ["categories", "badges"] {
            let dir = self.root.join(RESOURCES_DIR).join("icons").join(group);
            let Ok(entries) = std::fs::read_dir(&dir) else {
                continue;
            };
            let mut paths: Vec<PathBuf> = entries.flatten().map(|entry| entry.path()).collect();
            paths.sort();
            for path in paths {
                let relative = self.relative(&path);
                if path.extension().is_none_or(|ext| ext != "toml") {
                    self.stray.push(relative);
                    continue;
                }
                let key = path
                    .file_stem()
                    .map(|stem| stem.to_string_lossy().into_owned())
                    .unwrap_or_default();
                let parsed = std::fs::read_to_string(&path)
                    .map_err(|error| error.to_string())
                    .and_then(|text| {
                        toml::from_str::<IconFile>(&text).map_err(|error| error.to_string())
                    });
                match parsed {
                    Ok(file) => {
                        let map = if group == "categories" {
                            &mut self.icons.categories
                        } else {
                            &mut self.icons.badges
                        };
                        map.insert(key, file.icon);
                    }
                    Err(message) => self.problem(relative, message),
                }
            }
        }
    }

    fn read_palette(&mut self) {
        let path = self.root.join(RESOURCES_DIR).join("palette.toml");
        let Ok(text) = std::fs::read_to_string(&path) else {
            return;
        };
        match toml::from_str::<Palette>(&text) {
            Ok(palette) => self.palette = Some(palette),
            Err(error) => self.problem(self.relative(&path), error.to_string()),
        }
    }

    fn read_presets(&mut self) -> Result<()> {
        let presets = self.root.join(PRESETS_DIR);
        if !presets.is_dir() {
            self.problem(PRESETS_DIR, "folder is missing");
            return Ok(());
        }
        let mut by_stem: BTreeMap<(PathBuf, String), Slot> = BTreeMap::new();
        for entry in WalkDir::new(&presets).sort_by_file_name() {
            let entry = entry.map_err(|error| Error::io(&presets, error.into()))?;
            if !entry.file_type().is_file() {
                continue;
            }
            let path = entry.path();
            let relative = self.relative(path);
            let extension = path
                .extension()
                .map(|ext| ext.to_string_lossy().to_ascii_lowercase());
            let (Some(extension), Some(stem)) = (extension, path.file_stem()) else {
                self.stray.push(relative);
                continue;
            };
            if extension != "syx" && extension != "toml" {
                self.stray.push(relative);
                continue;
            }
            let stem = stem.to_string_lossy().into_owned();
            let folder = relative.parent().map(Path::to_path_buf).unwrap_or_default();
            let parts: Vec<String> = folder
                .strip_prefix(PRESETS_DIR)
                .unwrap_or(&folder)
                .components()
                .map(|part| part.as_os_str().to_string_lossy().into_owned())
                .collect();
            let slot = by_stem
                .entry((folder.clone(), stem.clone()))
                .or_insert_with(|| Slot {
                    folder: folder.clone(),
                    category_dir: parts.first().cloned().unwrap_or_default(),
                    collection: parts.get(1).cloned(),
                    extra_depth: parts.len().saturating_sub(2),
                    stem: stem.clone(),
                    syx: None,
                    toml: None,
                    toml_text: None,
                    meta: None,
                    bytes: None,
                    programs: Vec::new(),
                });
            if extension == "syx" {
                slot.syx = Some(relative.clone());
                match std::fs::read(path) {
                    Ok(bytes) => {
                        for program in syx::File::new(&bytes).programs() {
                            match program {
                                Ok(entry) => slot.programs.push((entry.slot, entry.program)),
                                Err(error) => self.problems.push(Finding::error(
                                    relative.clone(),
                                    format!("a frame did not parse: {error}"),
                                )),
                            }
                        }
                        slot.bytes = Some(bytes);
                    }
                    Err(error) => self
                        .problems
                        .push(Finding::error(relative, error.to_string())),
                }
            } else {
                slot.toml = Some(relative.clone());
                match std::fs::read_to_string(path) {
                    Ok(text) => {
                        match PatchMeta::parse(&text) {
                            Ok(meta) => slot.meta = Some(meta),
                            Err(message) => self.problems.push(Finding::error(relative, message)),
                        }
                        slot.toml_text = Some(text);
                    }
                    Err(error) => self
                        .problems
                        .push(Finding::error(relative, error.to_string())),
                }
            }
        }
        self.slots = by_stem.into_values().collect();
        Ok(())
    }

    fn read_demos(&mut self) -> Result<()> {
        let demos = self.root.join(DEMOS_DIR);
        if !demos.is_dir() {
            return Ok(());
        }
        for entry in WalkDir::new(&demos).sort_by_file_name() {
            let entry = entry.map_err(|error| Error::io(&demos, error.into()))?;
            if entry.file_type().is_file() {
                self.demo_files.push(self.relative(entry.path()));
            }
        }
        Ok(())
    }

    /// Files at the root and in folders no rule reads. Folders the tooling
    /// owns are left alone.
    fn find_strays(&mut self) -> Result<()> {
        const OWN_FOLDERS: &[&str] = &[
            PRESETS_DIR,
            DEMOS_DIR,
            RESOURCES_DIR,
            "crates",
            "tools",
            "docs",
            ".github",
            ".git",
            "target",
        ];
        const ROOT_FILES: &[&str] = &[
            "README.md",
            "CONTRIBUTING.md",
            "CHANGELOG.md",
            "LICENSE",
            "Cargo.toml",
            "Cargo.lock",
            TAXONOMY_FILE,
            ".gitignore",
            ".gitattributes",
            "taplo.toml",
            "rustfmt.toml",
            "clippy.toml",
            // Release outputs, gitignored, present while the workflow runs.
            "index.toml",
            "patches.tar.gz",
            "demos.tar.gz",
        ];
        let entries =
            std::fs::read_dir(&self.root).map_err(|error| Error::io(&self.root, error))?;
        let mut paths: Vec<PathBuf> = entries.flatten().map(|entry| entry.path()).collect();
        paths.sort();
        for path in paths {
            let name = path
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_default();
            if path.is_dir() {
                if !OWN_FOLDERS.contains(&name.as_str()) {
                    self.stray.push(self.relative(&path));
                }
            } else if !ROOT_FILES.contains(&name.as_str()) {
                self.stray.push(self.relative(&path));
            }
        }
        let resources = self.root.join(RESOURCES_DIR);
        if resources.is_dir() {
            for entry in WalkDir::new(&resources).sort_by_file_name() {
                let entry = entry.map_err(|error| Error::io(&resources, error.into()))?;
                if !entry.file_type().is_file() {
                    continue;
                }
                let relative = self.relative(entry.path());
                let allowed = relative == Path::new(RESOURCES_DIR).join("palette.toml")
                    || relative == Path::new(RESOURCES_DIR).join("README.md")
                    || relative.starts_with(Path::new(RESOURCES_DIR).join("icons"));
                if !allowed && !self.stray.contains(&relative) {
                    self.stray.push(relative);
                }
            }
        }
        Ok(())
    }

    /// A path relative to the root, or the path itself if it is elsewhere.
    #[must_use]
    pub fn relative(&self, path: &Path) -> PathBuf {
        path.strip_prefix(&self.root).unwrap_or(path).to_path_buf()
    }
}
