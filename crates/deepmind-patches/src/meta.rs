//! The patch `.toml`: what a person writes about a sound.
//!
//! Only what nothing else can know. Anything readable from the `.syx` is
//! derived by [`crate::derived`] and has no field here, because a typed fact
//! goes stale the moment someone edits the patch.
//!
//! ```toml
//! name    = "Acid Growl"          # equals the program's stored name
//! author  = "nyx"                 # a person or a handle
//! version = 1                     # bump whenever the .syx bytes change
//! about   = "Resonant 303-ish bass that opens under velocity."
//! licence = "CC-BY-4.0"           # SPDX identifier or "All rights reserved"
//!
//! genre  = ["techno"]
//! mood   = ["dark", "aggressive"]
//! timbre = ["gritty", "analogue"]
//! role   = ["bass"]
//!
//! icon = [".......", "..###..", ".#...#.", ".#...#.", ".#####.", ".#...#.", "......."]
//!
//! created  = 2026-09-21
//! model    = "DeepMind 12"
//! firmware = "1.1.5"
//! source   = "Reworked from a factory program."
//! tags     = ["303", "velocity"]
//!
//! [[demo]]
//! about = "Dry, C2 to C4."
//!
//! [[demo]]
//! variant = "mod wheel"
//! about   = "Wheel fully up."
//! ```

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::icon::Icon;
use crate::slug;
use crate::taxonomy::Axis;

/// The exact text accepted in `licence` for a patch its author keeps all
/// rights to.
pub const ALL_RIGHTS_RESERVED: &str = "All rights reserved";

/// Most demos a patch may declare.
pub const MAX_DEMOS: usize = 4;
/// Longest a demo variant label may be, in characters.
pub const MAX_VARIANT_CHARS: usize = 24;
/// Longest `about` may be, in characters. Two sentences, not an essay.
pub const MAX_ABOUT_CHARS: usize = 400;

/// One audio demo of a patch. The file name is computed, never typed.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Demo {
    /// What is different about this take: `mod wheel`, `aftertouch`, `arp`.
    /// Absent on the default demo, of which there is at most one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    /// One line on how it was played.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
}

/// Everything a patch `.toml` may hold.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchMeta {
    /// The program's stored name, sixteen characters at most.
    pub name: String,
    /// A person or a handle. Not an email address.
    pub author: String,
    /// Bumped whenever the `.syx` bytes change. Starts at 1.
    pub version: u32,
    /// What it sounds like and how to play it.
    pub about: String,
    /// SPDX identifier, or [`ALL_RIGHTS_RESERVED`].
    pub licence: String,
    /// Terms from `taxonomy.toml`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub genre: Vec<String>,
    /// Terms from `taxonomy.toml`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mood: Vec<String>,
    /// Terms from `taxonomy.toml`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub timbre: Vec<String>,
    /// Terms from `taxonomy.toml`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub role: Vec<String>,
    /// The patch's own icon. Falls back to the category icon when absent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
    /// When it was made. A TOML date.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<toml::value::Datetime>,
    /// Which instrument it was voiced on, as Behringer writes it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Firmware it was made and checked on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub firmware: Option<String>,
    /// Where it came from, if anywhere.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Free text, searched but never styled.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    /// Audio demos, first is the default.
    #[serde(default, rename = "demo", skip_serializing_if = "Vec::is_empty")]
    pub demos: Vec<Demo>,
    /// Fields this crate does not know. Empty in a valid file; kept so an older
    /// reader still opens a newer checkout.
    #[serde(default, flatten, skip_serializing)]
    pub unknown: toml::Table,
}

impl PatchMeta {
    /// Parses TOML text. Unknown fields land in [`PatchMeta::unknown`].
    pub fn parse(text: &str) -> std::result::Result<Self, String> {
        toml::from_str(text).map_err(|error| error.to_string())
    }

    /// Reads and parses a file.
    pub fn read(path: &Path) -> Result<Self> {
        let text = std::fs::read_to_string(path).map_err(|error| Error::io(path, error))?;
        Self::parse(&text).map_err(|message| Error::Toml {
            path: path.to_owned(),
            message,
        })
    }

    /// Serialises to TOML in the layout the repository uses.
    pub fn to_toml(&self) -> Result<String> {
        toml::to_string_pretty(self).map_err(|error| Error::Toml {
            path: Path::new(&slug::stem(&self.name, &self.author)).with_extension("toml"),
            message: error.to_string(),
        })
    }

    /// The filename stem both files share.
    #[must_use]
    pub fn stem(&self) -> String {
        slug::stem(&self.name, &self.author)
    }

    /// The file name of one demo, without a folder.
    #[must_use]
    pub fn demo_file_name(&self, demo: &Demo) -> String {
        format!(
            "{}.mp3",
            slug::demo_stem(&self.stem(), demo.variant.as_deref())
        )
    }

    /// The terms on one axis.
    #[must_use]
    pub fn terms(&self, axis: Axis) -> &[String] {
        match axis {
            Axis::Genre => &self.genre,
            Axis::Mood => &self.mood,
            Axis::Timbre => &self.timbre,
            Axis::Role => &self.role,
        }
    }

    /// Whether at least one vocabulary term is set on any axis.
    #[must_use]
    pub fn has_terms(&self) -> bool {
        Axis::ALL
            .into_iter()
            .any(|axis| !self.terms(axis).is_empty())
    }

    /// Whether the licence field is an SPDX identifier or the accepted
    /// reserved-rights text.
    #[must_use]
    pub fn licence_is_valid(&self) -> bool {
        self.licence == ALL_RIGHTS_RESERVED || spdx::license_id(&self.licence).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MINIMAL: &str = r#"
name = "Acid Growl"
author = "nyx"
version = 1
about = "Bass."
licence = "CC-BY-4.0"
role = ["bass"]
"#;

    #[test]
    fn minimal_parses() {
        let meta = PatchMeta::parse(MINIMAL).unwrap();
        assert_eq!(meta.stem(), "Acid Growl - nyx");
        assert!(meta.has_terms());
        assert!(meta.licence_is_valid());
        assert!(meta.unknown.is_empty());
        assert!(meta.icon.is_none());
    }

    #[test]
    fn unknown_fields_are_kept_not_refused() {
        let meta = PatchMeta::parse(&format!("{MINIMAL}\nfuture = 1\n")).unwrap();
        assert_eq!(meta.unknown.len(), 1);
    }

    #[test]
    fn demos_name_their_files() {
        let text =
            format!("{MINIMAL}\n[[demo]]\nabout = \"dry\"\n[[demo]]\nvariant = \"mod wheel\"\n");
        let meta = PatchMeta::parse(&text).unwrap();
        assert_eq!(meta.demo_file_name(&meta.demos[0]), "Acid Growl - nyx.mp3");
        assert_eq!(
            meta.demo_file_name(&meta.demos[1]),
            "Acid Growl - nyx (mod wheel).mp3"
        );
    }

    #[test]
    fn licences() {
        let mut meta = PatchMeta::parse(MINIMAL).unwrap();
        for ok in [
            "CC0-1.0",
            "CC-BY-4.0",
            "CC-BY-NC-SA-4.0",
            "MIT",
            ALL_RIGHTS_RESERVED,
        ] {
            meta.licence = ok.to_owned();
            assert!(meta.licence_is_valid(), "{ok}");
        }
        meta.licence = "all rights reserved".to_owned();
        assert!(!meta.licence_is_valid());
        meta.licence = "CC BY 4.0".to_owned();
        assert!(!meta.licence_is_valid());
    }

    #[test]
    fn round_trips_through_toml() {
        let meta = PatchMeta::parse(MINIMAL).unwrap();
        let text = meta.to_toml().unwrap();
        assert_eq!(PatchMeta::parse(&text).unwrap(), meta);
    }
}
