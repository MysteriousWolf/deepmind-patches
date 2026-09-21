//! What the program bytes say. Computed, never typed.

use deepmind_midi::ParamId;
use deepmind_midi::effect::Engine;
use deepmind_midi::program::Program;
use serde::{Deserialize, Serialize};

use crate::category::Category;

/// Facts a host filters and styles by, read out of the 242 parameters.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Derived {
    /// The category the program is stored with, if it is one of the twelve.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<Category>,
    /// The raw category byte, for the message when it is not one of the twelve.
    #[serde(skip)]
    pub category_value: u8,
    /// Full names of the algorithms loaded in the four effect engines, in
    /// engine order, skipping empty engines.
    pub effects: Vec<String>,
    /// Whether the arpeggiator is on.
    pub arp: bool,
    /// The arpeggiator mode, when it is on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arp_mode: Option<String>,
    /// The polyphony mode as the instrument labels it: `Poly`, `Unison 4`, `Mono`.
    pub polyphony: String,
    /// Voices stacked per note. 1 unless a unison mode is set.
    pub unison: u8,
    /// How many of the eight modulation routings have a source.
    pub routings: u8,
    /// Whether the control sequencer is enabled.
    pub sequencer: bool,
    /// Program transpose in semitones.
    pub transpose: i8,
}

impl Derived {
    /// Reads the facts out of a program.
    #[must_use]
    pub fn of(program: &Program) -> Self {
        let category_value = program.get(ParamId::ProgramCategory);
        let effects = Engine::ALL
            .iter()
            .filter_map(|engine| program.algorithm(*engine))
            .map(|algorithm| algorithm.full_name.to_owned())
            .collect();
        let arp = program.get(ParamId::ArpOnOff) != 0;
        let arp_mode = arp
            .then(|| ParamId::ArpMode.label(u16::from(program.get(ParamId::ArpMode))))
            .flatten()
            .map(str::to_owned);
        let polyphony_value = program.get(ParamId::PolyphonyMode);
        let polyphony = ParamId::PolyphonyMode
            .label(u16::from(polyphony_value))
            .map_or_else(|| format!("value {polyphony_value}"), str::to_owned);
        let unison = polyphony
            .strip_prefix("Unison ")
            .and_then(|count| count.parse().ok())
            .unwrap_or(1);
        let routings = [
            ParamId::Mod1Source,
            ParamId::Mod2Source,
            ParamId::Mod3Source,
            ParamId::Mod4Source,
            ParamId::Mod5Source,
            ParamId::Mod6Source,
            ParamId::Mod7Source,
            ParamId::Mod8Source,
        ]
        .into_iter()
        .filter(|source| program.get(*source) != 0)
        .count();
        Self {
            category: Category::from_value(category_value),
            category_value,
            effects,
            arp,
            arp_mode,
            polyphony,
            unison,
            routings: u8::try_from(routings).unwrap_or(u8::MAX),
            sequencer: program.get(ParamId::CtrlSequencerEnable) != 0,
            transpose: program.transpose(),
        }
    }
}

#[cfg(test)]
mod tests {
    use deepmind_midi::ids::ProtocolVersion;

    use super::*;

    #[test]
    fn reads_a_blank_program() {
        let mut program = Program::new(ProtocolVersion::V7);
        program
            .set(ParamId::ProgramCategory, Category::Pad.value())
            .unwrap();
        program.set(ParamId::ArpOnOff, 1).unwrap();
        program.set(ParamId::Mod1Source, 3).unwrap();
        program.set(ParamId::PolyphonyMode, 3).unwrap();
        let derived = Derived::of(&program);
        assert_eq!(derived.category, Some(Category::Pad));
        assert!(derived.arp);
        assert!(derived.arp_mode.is_some());
        assert_eq!(derived.routings, 1);
        assert_eq!(derived.polyphony, "Unison 4");
        assert_eq!(derived.unison, 4);
        assert!(!derived.sequencer);
    }
}
