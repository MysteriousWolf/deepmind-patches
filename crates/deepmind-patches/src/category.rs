//! The instrument's twelve program categories.
//!
//! The category is byte 240 of the program, so it is the one classification
//! nobody types: the folder a patch sits in must agree with the file. `None`
//! and the four `User` slots are stored values but not categories here, and a
//! patch carrying one is refused until a category is set on the instrument.
//!
//! Each category has two names. [`Category::name`] is the readable one, which
//! is the folder, the icon file and what the index writes: `Sound Effects`.
//! [`Category::label`] is the instrument's own short form: `SFX`.

use std::fmt;

use deepmind_midi::ParamId;
use deepmind_midi::program::Program;
use serde::{Deserialize, Serialize};

/// One of the twelve categories, in the instrument's order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Category {
    /// Plays the bottom.
    Bass,
    /// Held chords.
    #[serde(rename = "Pads")]
    Pad,
    /// Plays the tune.
    #[serde(rename = "Leads")]
    Lead,
    /// Monophonic, not otherwise a bass or a lead.
    #[serde(rename = "Monophonic")]
    Mono,
    /// Polyphonic, not otherwise a pad.
    #[serde(rename = "Polyphonic")]
    Poly,
    /// One hit, chordal.
    #[serde(rename = "Stabs")]
    Stab,
    /// Sound effects.
    #[serde(rename = "Sound Effects")]
    Sfx,
    /// The arpeggiator is the point.
    #[serde(rename = "Arpeggios")]
    Arp,
    /// The sequencer is the point.
    #[serde(rename = "Sequences")]
    Seq,
    /// Drums, hits, clicks.
    #[serde(rename = "Percussion")]
    Perc,
    /// Drifting, atmospheric.
    Ambient,
    /// Patches built around the modulation matrix.
    Modular,
}

impl Category {
    /// Every category, in the instrument's order.
    pub const ALL: [Self; 12] = [
        Self::Bass,
        Self::Pad,
        Self::Lead,
        Self::Mono,
        Self::Poly,
        Self::Stab,
        Self::Sfx,
        Self::Arp,
        Self::Seq,
        Self::Perc,
        Self::Ambient,
        Self::Modular,
    ];

    /// The category a stored value names, or `None` for `None` and `User-1`
    /// through `User-4`.
    #[must_use]
    pub const fn from_value(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::Bass),
            2 => Some(Self::Pad),
            3 => Some(Self::Lead),
            4 => Some(Self::Mono),
            5 => Some(Self::Poly),
            6 => Some(Self::Stab),
            7 => Some(Self::Sfx),
            8 => Some(Self::Arp),
            9 => Some(Self::Seq),
            10 => Some(Self::Perc),
            11 => Some(Self::Ambient),
            12 => Some(Self::Modular),
            _ => None,
        }
    }

    /// The value the program stores for this category.
    #[must_use]
    pub const fn value(self) -> u8 {
        match self {
            Self::Bass => 1,
            Self::Pad => 2,
            Self::Lead => 3,
            Self::Mono => 4,
            Self::Poly => 5,
            Self::Stab => 6,
            Self::Sfx => 7,
            Self::Arp => 8,
            Self::Seq => 9,
            Self::Perc => 10,
            Self::Ambient => 11,
            Self::Modular => 12,
        }
    }

    /// The readable name: the folder under `presets/`, the icon file, and
    /// what the index writes.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Bass => "Bass",
            Self::Pad => "Pads",
            Self::Lead => "Leads",
            Self::Mono => "Monophonic",
            Self::Poly => "Polyphonic",
            Self::Stab => "Stabs",
            Self::Sfx => "Sound Effects",
            Self::Arp => "Arpeggios",
            Self::Seq => "Sequences",
            Self::Perc => "Percussion",
            Self::Ambient => "Ambient",
            Self::Modular => "Modular",
        }
    }

    /// The short label the instrument's display uses: `SFX`, `Perc`.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Bass => "Bass",
            Self::Pad => "Pad",
            Self::Lead => "Lead",
            Self::Mono => "Mono",
            Self::Poly => "Poly",
            Self::Stab => "Stab",
            Self::Sfx => "SFX",
            Self::Arp => "Arp",
            Self::Seq => "Seq",
            Self::Perc => "Perc",
            Self::Ambient => "Ambient",
            Self::Modular => "Modular",
        }
    }

    /// What the category is for, in a few words.
    #[must_use]
    pub const fn about(self) -> &'static str {
        match self {
            Self::Bass => "Plays the bottom",
            Self::Pad => "Held chords",
            Self::Lead => "Plays the tune",
            Self::Mono => "One voice, not a bass or a lead",
            Self::Poly => "Many voices, not a pad",
            Self::Stab => "One hit, chordal",
            Self::Sfx => "Not an instrument",
            Self::Arp => "The arpeggiator is the point",
            Self::Seq => "The sequencer is the point",
            Self::Perc => "Drums, hits, clicks",
            Self::Ambient => "Drifting, atmospheric",
            Self::Modular => "Built around the modulation matrix",
        }
    }

    /// The category a folder name refers to. Case-sensitive.
    #[must_use]
    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|category| category.name() == name)
    }

    /// The category an instrument label refers to. Case-sensitive.
    #[must_use]
    pub fn from_label(label: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|category| category.label() == label)
    }

    /// The category a program is stored with.
    #[must_use]
    pub fn of(program: &Program) -> Option<Self> {
        Self::from_value(program.get(ParamId::ProgramCategory))
    }

    /// The label the instrument shows for a stored value, including the ones
    /// that are not categories here.
    #[must_use]
    pub fn label_of_value(value: u8) -> String {
        ParamId::ProgramCategory
            .label(u16::from(value))
            .map_or_else(|| format!("value {value}"), str::to_owned)
    }
}

impl std::str::FromStr for Category {
    type Err = String;

    fn from_str(name: &str) -> std::result::Result<Self, Self::Err> {
        Self::from_name(name)
            .or_else(|| Self::from_label(name))
            .ok_or_else(|| {
                let names: Vec<&str> = Self::ALL.iter().map(|category| category.name()).collect();
                format!("{name:?} is not a category; one of {}", names.join(", "))
            })
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn values_round_trip() {
        for category in Category::ALL {
            assert_eq!(Category::from_value(category.value()), Some(category));
            assert_eq!(Category::from_name(category.name()), Some(category));
            assert_eq!(Category::from_label(category.label()), Some(category));
            assert_eq!(Category::label_of_value(category.value()), category.label());
        }
        assert_eq!(Category::from_value(0), None);
        assert_eq!(Category::from_value(13), None);
    }

    #[test]
    fn sfx_serialises_as_its_folder_name() {
        let text = toml::to_string(&toml::Table::from_iter([(
            "c".to_owned(),
            toml::Value::try_from(Category::Sfx).unwrap(),
        )]))
        .unwrap();
        assert_eq!(text.trim(), "c = \"Sound Effects\"");
    }
}
