//! The controlled vocabularies in `taxonomy.toml`.
//!
//! Four axes, each answering a different question: `genre` is what music it is
//! for, `mood` is how it feels, `timbre` is what it sounds like, `role` is what
//! it does in a track. A term not in the file fails validation. Adding one is a
//! pull request.

use std::collections::BTreeMap;
use std::fmt;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

/// One of the four vocabulary axes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Axis {
    /// What music it is for.
    Genre,
    /// How it feels.
    Mood,
    /// What it sounds like.
    Timbre,
    /// What it does in a track.
    Role,
}

impl Axis {
    /// Every axis, in the order the files list them.
    pub const ALL: [Self; 4] = [Self::Genre, Self::Mood, Self::Timbre, Self::Role];

    /// The key used in TOML.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Genre => "genre",
            Self::Mood => "mood",
            Self::Timbre => "timbre",
            Self::Role => "role",
        }
    }
}

impl fmt::Display for Axis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// Term to one-line description, per axis.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Taxonomy {
    /// What music it is for.
    #[serde(default)]
    pub genre: BTreeMap<String, String>,
    /// How it feels.
    #[serde(default)]
    pub mood: BTreeMap<String, String>,
    /// What it sounds like.
    #[serde(default)]
    pub timbre: BTreeMap<String, String>,
    /// What it does in a track.
    #[serde(default)]
    pub role: BTreeMap<String, String>,
}

impl Taxonomy {
    /// Parses `taxonomy.toml` text.
    pub fn parse(text: &str) -> Result<Self> {
        toml::from_str(text).map_err(|error| Error::Toml {
            path: crate::TAXONOMY_FILE.into(),
            message: error.to_string(),
        })
    }

    /// Reads and parses a file.
    pub fn read(path: &Path) -> Result<Self> {
        let text = std::fs::read_to_string(path).map_err(|error| Error::io(path, error))?;
        toml::from_str(&text).map_err(|error| Error::Toml {
            path: path.to_owned(),
            message: error.to_string(),
        })
    }

    /// The terms on one axis.
    #[must_use]
    pub const fn axis(&self, axis: Axis) -> &BTreeMap<String, String> {
        match axis {
            Axis::Genre => &self.genre,
            Axis::Mood => &self.mood,
            Axis::Timbre => &self.timbre,
            Axis::Role => &self.role,
        }
    }

    /// Whether a term exists on an axis.
    #[must_use]
    pub fn contains(&self, axis: Axis, term: &str) -> bool {
        self.axis(axis).contains_key(term)
    }

    /// Terms that are not lowercase ASCII words, which the file forbids.
    pub fn malformed_terms(&self) -> impl Iterator<Item = (Axis, &str)> {
        Axis::ALL.into_iter().flat_map(move |axis| {
            self.axis(axis)
                .keys()
                .filter(|term| !is_term(term))
                .map(move |term| (axis, term.as_str()))
        })
    }
}

/// Whether text is a valid term: lowercase ASCII letters, digits and `-`.
#[must_use]
pub fn is_term(text: &str) -> bool {
    !text.is_empty()
        && text.chars().all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
        })
        && !text.starts_with('-')
        && !text.ends_with('-')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_looks_up() {
        let taxonomy =
            Taxonomy::parse("[genre]\ntechno = \"Machine music.\"\n[role]\nbass = \"Bottom.\"\n")
                .unwrap();
        assert!(taxonomy.contains(Axis::Genre, "techno"));
        assert!(!taxonomy.contains(Axis::Mood, "techno"));
        assert_eq!(taxonomy.malformed_terms().count(), 0);
    }

    #[test]
    fn unknown_axis_is_refused() {
        assert!(Taxonomy::parse("[colour]\nred = \"x\"\n").is_err());
    }

    #[test]
    fn terms_are_lowercase_words() {
        assert!(is_term("drum-and-bass"));
        assert!(!is_term("Techno"));
        assert!(!is_term("-x"));
        assert!(!is_term(""));
    }
}
