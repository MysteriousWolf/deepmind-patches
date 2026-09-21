//! The identity of a sound.
//!
//! A fingerprint is the SHA-256 of the program's parameter bytes with the name
//! and the category left out. A program dumped from an instrument after loading
//! matches the file it came from: bank, slot and sysex framing are not part of
//! the program, and renaming or recategorising on the front panel does not make
//! it a different sound. Any other edit does.
//!
//! The index carries every fingerprint each patch version ever had, so a host
//! that dumps a program can name the patch and the version, and say whether a
//! newer one exists.

use std::fmt;
use std::str::FromStr;

use deepmind_midi::ParamId;
use deepmind_midi::program::Program;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sha2::{Digest, Sha256};

use crate::error::Error;

/// Parameters excluded from the fingerprint: the seventeen name bytes and the
/// category.
const EXCLUDED: std::ops::RangeInclusive<u8> =
    (ParamId::ProgramNameChar1 as u8)..=(ParamId::ProgramCategory as u8);

/// SHA-256 over what makes a program sound the way it does.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Fingerprint([u8; 32]);

impl Fingerprint {
    /// Fingerprints a program.
    #[must_use]
    pub fn of(program: &Program) -> Self {
        let mut hasher = Sha256::new();
        for (parameter, value) in program.values() {
            if !EXCLUDED.contains(&(parameter as u8)) {
                hasher.update([value]);
            }
        }
        Self(hasher.finalize().into())
    }

    /// The raw digest.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Lowercase hex, 64 characters.
    #[must_use]
    pub fn to_hex(&self) -> String {
        hex(&self.0)
    }
}

/// SHA-256 of a whole file, for download integrity. Hex.
#[must_use]
pub fn sha256_hex(bytes: &[u8]) -> String {
    hex(&Sha256::digest(bytes))
}

fn hex(bytes: &[u8]) -> String {
    use std::fmt::Write as _;
    bytes
        .iter()
        .fold(String::with_capacity(bytes.len() * 2), |mut out, byte| {
            let _ = write!(out, "{byte:02x}");
            out
        })
}

impl FromStr for Fingerprint {
    type Err = Error;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        if text.len() != 64 {
            return Err(Error::Fingerprint(format!(
                "expected 64 hex digits, found {}",
                text.len()
            )));
        }
        let mut bytes = [0u8; 32];
        for (index, byte) in bytes.iter_mut().enumerate() {
            let pair = text
                .get(index * 2..index * 2 + 2)
                .ok_or_else(|| Error::Fingerprint("not ASCII".to_owned()))?;
            *byte = u8::from_str_radix(pair, 16)
                .map_err(|_| Error::Fingerprint(format!("{pair:?} is not hex")))?;
        }
        Ok(Self(bytes))
    }
}

impl fmt::Display for Fingerprint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_hex())
    }
}

impl fmt::Debug for Fingerprint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Fingerprint({})", self.to_hex())
    }
}

impl Serialize for Fingerprint {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_hex())
    }
}

impl<'de> Deserialize<'de> for Fingerprint {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = String::deserialize(deserializer)?;
        text.parse().map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use deepmind_midi::ids::ProtocolVersion;
    use deepmind_midi::program::ProgramName;

    use super::*;

    #[test]
    fn name_and_category_do_not_change_it() {
        let mut a = Program::new(ProtocolVersion::V7);
        let before = Fingerprint::of(&a);
        a.set_name(ProgramName::new("Renamed").unwrap());
        a.set(ParamId::ProgramCategory, 5).unwrap();
        assert_eq!(Fingerprint::of(&a), before);
        a.set(ParamId::VcfResonance, 100).unwrap();
        assert_ne!(Fingerprint::of(&a), before);
    }

    #[test]
    fn hex_round_trip() {
        let fingerprint = Fingerprint::of(&Program::new(ProtocolVersion::V6));
        let text = fingerprint.to_hex();
        assert_eq!(text.len(), 64);
        assert_eq!(text.parse::<Fingerprint>().unwrap(), fingerprint);
        assert!("abc".parse::<Fingerprint>().is_err());
    }
}
