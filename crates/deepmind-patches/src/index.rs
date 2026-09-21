//! The generated `index.toml`.
//!
//! Built by CI from a validated checkout and shipped with every release. An
//! application reads this one file to browse, search, match a dumped program
//! back to a patch and version, and fetch a single `.syx` pinned to the commit
//! the index was built from.
//!
//! Nobody edits it by hand and nobody commits it.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::INDEX_SCHEMA;
use crate::category::Category;
use crate::error::{Error, Result};
use crate::fingerprint::Fingerprint;
use crate::icon::Icon;
use crate::library::{Icons, Library, Palette};
use crate::taxonomy::Taxonomy;
use crate::version::LibraryVersion;

/// What the index was built from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Catalogue {
    /// Layout of this file. A reader refuses a number above [`INDEX_SCHEMA`].
    pub schema: u32,
    /// The library version this index describes.
    pub version: LibraryVersion,
    /// The commit every `file` path is valid at.
    pub commit: String,
    /// When it was built, RFC 3339 in UTC.
    pub built: String,
    /// How many patches follow.
    pub patches: usize,
}

/// One version a patch has had, and the sound it was.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryEntry {
    /// The `version` field at that time.
    pub version: u32,
    /// The fingerprint at that version.
    pub fingerprint: Fingerprint,
}

/// One demo, with the file resolved.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndexDemo {
    /// Path relative to the repository root.
    pub file: String,
    /// The variant label, absent on the default demo.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    /// How it was played.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
}

/// One patch as the index describes it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndexPatch {
    /// `bass/acid-growl-nyx`.
    pub id: String,
    /// The stored name.
    pub name: String,
    /// Who made it.
    pub author: String,
    /// Current version of the sound.
    pub version: u32,
    /// What it sounds like.
    pub about: String,
    /// The folder and the program byte.
    pub category: Category,
    /// The collection folder, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collection: Option<String>,
    /// The `.syx` path relative to the repository root.
    pub file: String,
    /// SHA-256 of the file, hex.
    pub sha256: String,
    /// Identity of the sound at this version.
    pub fingerprint: Fingerprint,
    /// The licence text the author wrote.
    pub licence: String,
    /// Vocabulary terms.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub genre: Vec<String>,
    /// Vocabulary terms.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mood: Vec<String>,
    /// Vocabulary terms.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub timbre: Vec<String>,
    /// Vocabulary terms.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub role: Vec<String>,
    /// Free text.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    /// The icon to draw: the patch's own or its category's.
    pub icon: Icon,
    /// Whether `icon` is the patch's own rather than the category fallback.
    pub own_icon: bool,
    /// When it was made.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<toml::value::Datetime>,
    /// Which instrument it was voiced on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Firmware it was checked on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub firmware: Option<String>,
    /// Where it came from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Demos, default first.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub demos: Vec<IndexDemo>,
    /// `Insert`, `Send` or `Bypass`.
    pub fx_mode: String,
    /// Effect algorithms in engine order, empty when bypassed.
    pub effects: Vec<String>,
    /// Arpeggiator on.
    pub arp: bool,
    /// Arpeggiator mode, when on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arp_mode: Option<String>,
    /// Polyphony mode label.
    pub polyphony: String,
    /// Voices per note.
    pub unison: u8,
    /// Modulation routings in use.
    pub routings: u8,
    /// Control sequencer enabled.
    pub sequencer: bool,
    /// Transpose in semitones.
    pub transpose: i8,
    /// Every version this patch has had, oldest first, current last.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub history: Vec<HistoryEntry>,
}

/// The whole file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Index {
    /// Provenance.
    pub catalogue: Catalogue,
    /// The vocabularies, so a reader needs no second file.
    pub taxonomy: Taxonomy,
    /// Category and badge icons.
    pub icons: Icons,
    /// Named colours, if the checkout has them.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub palette: Palette,
    /// Every patch, sorted by id.
    #[serde(rename = "patch", default)]
    pub patches: Vec<IndexPatch>,
}

/// What a fingerprint lookup found.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Match<'a> {
    /// The patch.
    pub patch: &'a IndexPatch,
    /// The version whose fingerprint matched.
    pub version: u32,
    /// Whether that is the patch's current version.
    pub latest: bool,
}

impl Index {
    /// Builds the index from a valid library.
    ///
    /// `history` maps patch id to earlier versions, oldest first; the current
    /// version is appended when missing. A patch with no entry gets a history
    /// of just its current version.
    #[must_use]
    pub fn build(
        library: &Library,
        version: LibraryVersion,
        commit: &str,
        built: &str,
        history: &BTreeMap<String, Vec<HistoryEntry>>,
    ) -> Self {
        let patches: Vec<IndexPatch> = library
            .patches()
            .iter()
            .map(|patch| {
                let mut entries: Vec<HistoryEntry> =
                    history.get(&patch.id).cloned().unwrap_or_default();
                entries.retain(|entry| entry.version < patch.meta.version);
                entries.push(HistoryEntry {
                    version: patch.meta.version,
                    fingerprint: patch.fingerprint,
                });
                let meta = &patch.meta;
                let derived = &patch.derived;
                IndexPatch {
                    id: patch.id.clone(),
                    name: meta.name.clone(),
                    author: meta.author.clone(),
                    version: meta.version,
                    about: meta.about.trim().to_owned(),
                    category: patch.category,
                    collection: patch.collection.clone(),
                    file: patch.syx_path.to_string_lossy().replace('\\', "/"),
                    sha256: patch.sha256.clone(),
                    fingerprint: patch.fingerprint,
                    licence: meta.licence.clone(),
                    genre: meta.genre.clone(),
                    mood: meta.mood.clone(),
                    timbre: meta.timbre.clone(),
                    role: meta.role.clone(),
                    tags: meta.tags.clone(),
                    icon: library.icon_for(patch),
                    own_icon: meta.icon.is_some(),
                    created: meta.created,
                    model: meta.model.clone(),
                    firmware: meta.firmware.clone(),
                    source: meta.source.clone(),
                    demos: meta
                        .demos
                        .iter()
                        .map(|demo| IndexDemo {
                            file: patch.demo_path(demo).to_string_lossy().replace('\\', "/"),
                            variant: demo.variant.clone(),
                            about: demo.about.clone(),
                        })
                        .collect(),
                    fx_mode: derived.fx_mode.clone(),
                    effects: derived.effects.clone(),
                    arp: derived.arp,
                    arp_mode: derived.arp_mode.clone(),
                    polyphony: derived.polyphony.clone(),
                    unison: derived.unison,
                    routings: derived.routings,
                    sequencer: derived.sequencer,
                    transpose: derived.transpose,
                    history: entries,
                }
            })
            .collect();
        Self {
            catalogue: Catalogue {
                schema: INDEX_SCHEMA,
                version,
                commit: commit.to_owned(),
                built: built.to_owned(),
                patches: patches.len(),
            },
            taxonomy: library.taxonomy.clone(),
            icons: library.icons.clone(),
            palette: library.palette.clone().unwrap_or_default(),
            patches,
        }
    }

    /// Parses an index, refusing a schema this crate does not read.
    pub fn parse(text: &str) -> Result<Self> {
        #[derive(Deserialize)]
        struct Head {
            catalogue: HeadCatalogue,
        }
        #[derive(Deserialize)]
        struct HeadCatalogue {
            schema: u32,
        }
        let head: Head = toml::from_str(text).map_err(|error| Error::Toml {
            path: "index.toml".into(),
            message: error.to_string(),
        })?;
        if head.catalogue.schema > INDEX_SCHEMA {
            return Err(Error::Schema {
                found: head.catalogue.schema,
                supported: INDEX_SCHEMA,
            });
        }
        toml::from_str(text).map_err(|error| Error::Toml {
            path: "index.toml".into(),
            message: error.to_string(),
        })
    }

    /// Serialises the index.
    pub fn to_toml(&self) -> Result<String> {
        toml::to_string_pretty(self).map_err(|error| Error::Toml {
            path: "index.toml".into(),
            message: error.to_string(),
        })
    }

    /// One patch by id.
    #[must_use]
    pub fn get(&self, id: &str) -> Option<&IndexPatch> {
        self.patches.iter().find(|patch| patch.id == id)
    }

    /// The patch and version a fingerprint belongs to, if any version of any
    /// patch ever had it.
    #[must_use]
    pub fn lookup(&self, fingerprint: &Fingerprint) -> Option<Match<'_>> {
        for patch in &self.patches {
            if patch.fingerprint == *fingerprint {
                return Some(Match {
                    patch,
                    version: patch.version,
                    latest: true,
                });
            }
            if let Some(entry) = patch
                .history
                .iter()
                .find(|entry| entry.fingerprint == *fingerprint)
            {
                return Some(Match {
                    patch,
                    version: entry.version,
                    latest: entry.version == patch.version,
                });
            }
        }
        None
    }

    /// Patches whose stored name equals `name`. What to try when a fingerprint
    /// misses: the program was probably edited after loading.
    pub fn by_name<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &'a IndexPatch> + 'a {
        self.patches.iter().filter(move |patch| patch.name == name)
    }

    /// The patches in one category.
    pub fn in_category(&self, category: Category) -> impl Iterator<Item = &IndexPatch> {
        self.patches
            .iter()
            .filter(move |patch| patch.category == category)
    }

    /// The URL of one file at the commit this index was built from.
    #[must_use]
    pub fn raw_url(&self, owner_repo: &str, file: &str) -> String {
        raw_url(owner_repo, &self.catalogue.commit, file)
    }
}

/// `https://raw.githubusercontent.com/<owner/repo>/<commit>/<file>`, with the
/// path percent-encoded.
#[must_use]
pub fn raw_url(owner_repo: &str, commit: &str, file: &str) -> String {
    let path: Vec<String> = file.split('/').map(percent_encode).collect();
    format!(
        "https://raw.githubusercontent.com/{owner_repo}/{commit}/{}",
        path.join("/")
    )
}

fn percent_encode(segment: &str) -> String {
    use std::fmt::Write as _;
    let mut out = String::with_capacity(segment.len());
    for byte in segment.bytes() {
        let keep = byte.is_ascii_alphanumeric() || b"-_.~".contains(&byte);
        if keep {
            out.push(char::from(byte));
        } else {
            let _ = write!(out, "%{byte:02X}");
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn urls_are_encoded() {
        assert_eq!(
            raw_url("o/r", "abc", "presets/Bass/Acid Growl - nyx.syx"),
            "https://raw.githubusercontent.com/o/r/abc/presets/Bass/Acid%20Growl%20-%20nyx.syx"
        );
    }

    #[test]
    fn newer_schema_is_refused() {
        let text = "[catalogue]\nschema = 99\nversion = \"26.0.0\"\ncommit = \"x\"\nbuilt = \"y\"\npatches = 0\n";
        assert!(matches!(
            Index::parse(text),
            Err(Error::Schema { found: 99, .. })
        ));
    }
}
