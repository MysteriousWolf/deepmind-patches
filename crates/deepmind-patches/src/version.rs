//! Library versioning: `yy.release.patch`.
//!
//! - `yy` is the two-digit year of the release. It resets the other two.
//! - `release` bumps when presets are added.
//! - `patch` bumps when only existing presets, demos, taxonomy or docs change.
//!
//! The version lives in git tags (`v26.1.0`), never in a file, so a content
//! pull request never edits a shared version line and never conflicts with
//! another one. The release workflow computes the next version from what
//! changed since the last tag.
//!
//! The crate that reads all this has its own version on the same scheme, on
//! its own counter, because code and content change at different times.

use std::fmt;
use std::path::Path;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::error::Error;

/// A library version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct LibraryVersion {
    /// Two-digit year, `26` for 2026.
    pub year: u32,
    /// Content releases this year.
    pub release: u32,
    /// Patches since the last release.
    pub patch: u32,
}

/// What kind of bump a change needs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Bump {
    /// Nothing that ships changed.
    None,
    /// Existing content changed.
    Patch,
    /// Content was added.
    Release,
    /// First release of a new year.
    Year,
}

impl LibraryVersion {
    /// The tag a version is released under: `v26.1.0`.
    #[must_use]
    pub fn tag(&self) -> String {
        format!("v{self}")
    }

    /// Parses a tag, with or without the `v`.
    pub fn from_tag(tag: &str) -> Result<Self, Error> {
        tag.strip_prefix('v').unwrap_or(tag).parse()
    }

    /// The next version after a bump, given the current two-digit year.
    ///
    /// A new year turns any content bump into `yy.0.0`; `Bump::None` returns
    /// the same version.
    #[must_use]
    pub fn next(self, bump: Bump, year: u32) -> Self {
        match bump {
            Bump::None => self,
            _ if year != self.year || bump == Bump::Year => Self {
                year,
                release: 0,
                patch: 0,
            },
            Bump::Patch => Self {
                patch: self.patch + 1,
                ..self
            },
            Bump::Release => Self {
                release: self.release + 1,
                patch: 0,
                ..self
            },
            Bump::Year => unreachable!("handled above"),
        }
    }

    /// The bump that takes `self` to `next`, if `next` is exactly one step
    /// away.
    ///
    /// This is the rule for a version that lives in a file, like the crate's:
    /// a change bumps the patch, the release, or starts the given year at
    /// `yy.0.0`. Anything else, including a skipped number, going backwards,
    /// or a year that is not this year, is an error saying what was expected.
    pub fn step_to(self, next: Self, year: u32) -> Result<Bump, Error> {
        let candidates = [
            (Bump::Patch, self.next(Bump::Patch, self.year)),
            (Bump::Release, self.next(Bump::Release, self.year)),
            (Bump::Year, self.next(Bump::Year, year)),
        ];
        candidates
            .iter()
            .find(|(bump, version)| *version == next && (*bump != Bump::Year || year > self.year))
            .map(|(bump, _)| *bump)
            .ok_or_else(|| {
                let mut expected: Vec<String> = candidates[..2]
                    .iter()
                    .map(|(_, version)| version.to_string())
                    .collect();
                if year > self.year {
                    expected.push(candidates[2].1.to_string());
                }
                Error::Version(format!(
                    "{next} is not one step from {self}; expected {}",
                    expected.join(", ")
                ))
            })
    }
}

impl FromStr for LibraryVersion {
    type Err = Error;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut parts = text.split('.').map(|part| {
            if part.is_empty()
                || part.len() > 1 && part.starts_with('0')
                || !part.bytes().all(|b| b.is_ascii_digit())
            {
                Err(Error::Version(format!("{text:?} is not yy.release.patch")))
            } else {
                part.parse::<u32>()
                    .map_err(|_| Error::Version(format!("{text:?} is not yy.release.patch")))
            }
        });
        let (Some(year), Some(release), Some(patch), None) =
            (parts.next(), parts.next(), parts.next(), parts.next())
        else {
            return Err(Error::Version(format!("{text:?} is not yy.release.patch")));
        };
        let (year, release, patch) = (year?, release?, patch?);
        if year > 99 {
            return Err(Error::Version(format!("{text:?}: year is two digits")));
        }
        Ok(Self {
            year,
            release,
            patch,
        })
    }
}

impl fmt::Display for LibraryVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.year, self.release, self.patch)
    }
}

impl Serialize for LibraryVersion {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for LibraryVersion {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer)?
            .parse()
            .map_err(serde::de::Error::custom)
    }
}

/// How a changed path was changed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Change {
    /// The path is new.
    Added,
    /// The path exists on both sides and differs.
    Modified,
    /// The path is gone.
    Deleted,
    /// The path moved. Counts as modified.
    Renamed,
}

/// The bump a set of changed paths requires.
///
/// A new `.syx` under `presets/` is a release. Anything else that ships, which
/// is everything under `presets/`, `demos/` and `resources/`, plus
/// `taxonomy.toml`, is a patch. Tooling, workflows and docs need no bump.
#[must_use]
pub fn classify<'a>(changes: impl IntoIterator<Item = (Change, &'a Path)>) -> Bump {
    let mut bump = Bump::None;
    for (change, path) in changes {
        let in_presets = path.starts_with(crate::PRESETS_DIR);
        let is_syx = path
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("syx"));
        if in_presets && is_syx && change == Change::Added {
            return Bump::Release;
        }
        let ships = in_presets
            || path.starts_with(crate::DEMOS_DIR)
            || path.starts_with(crate::RESOURCES_DIR)
            || path == Path::new(crate::TAXONOMY_FILE);
        if ships {
            bump = bump.max(Bump::Patch);
        }
    }
    bump
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_prints() {
        let version: LibraryVersion = "26.3.1".parse().unwrap();
        assert_eq!((version.year, version.release, version.patch), (26, 3, 1));
        assert_eq!(version.tag(), "v26.3.1");
        assert_eq!(LibraryVersion::from_tag("v26.3.1").unwrap(), version);
        for bad in ["26.3", "2026.1.0", "26.01.0", "v26.1.0", "26.1.0.0", ""] {
            assert!(bad.parse::<LibraryVersion>().is_err(), "{bad}");
        }
    }

    #[test]
    fn bumps() {
        let version: LibraryVersion = "26.3.1".parse().unwrap();
        assert_eq!(version.next(Bump::None, 26).to_string(), "26.3.1");
        assert_eq!(version.next(Bump::Patch, 26).to_string(), "26.3.2");
        assert_eq!(version.next(Bump::Release, 26).to_string(), "26.4.0");
        assert_eq!(version.next(Bump::Patch, 27).to_string(), "27.0.0");
        assert_eq!(version.next(Bump::Year, 26).to_string(), "26.0.0");
    }

    #[test]
    fn steps() {
        let version: LibraryVersion = "26.3.1".parse().unwrap();
        let step = |next: &str, year| version.step_to(next.parse().unwrap(), year);
        assert_eq!(step("26.3.2", 26).unwrap(), Bump::Patch);
        assert_eq!(step("26.4.0", 26).unwrap(), Bump::Release);
        assert_eq!(step("27.0.0", 27).unwrap(), Bump::Year);
        // Still the same year on the calendar and in the version.
        assert_eq!(step("26.3.2", 27).unwrap(), Bump::Patch);
        for (bad, year) in [
            ("26.3.1", 26),
            ("26.3.3", 26),
            ("26.4.1", 26),
            ("26.5.0", 26),
            ("26.3.0", 26),
            ("25.0.0", 26),
            ("27.0.0", 26),
            ("28.0.0", 27),
            ("27.1.0", 27),
        ] {
            assert!(step(bad, year).is_err(), "{bad} in {year}");
        }
        let message = step("26.9.9", 27).unwrap_err().to_string();
        assert!(message.contains("26.3.2, 26.4.0, 27.0.0"), "{message}");
        let message = step("26.9.9", 26).unwrap_err().to_string();
        assert!(message.ends_with("26.3.2, 26.4.0"), "{message}");
    }

    #[test]
    fn classifies_changes() {
        let p = Path::new;
        assert_eq!(
            classify([(Change::Added, p("presets/Bass/X - y.syx"))]),
            Bump::Release
        );
        assert_eq!(
            classify([(Change::Modified, p("presets/Bass/X - y.syx"))]),
            Bump::Patch
        );
        assert_eq!(
            classify([(Change::Added, p("presets/Bass/X - y.toml"))]),
            Bump::Patch
        );
        assert_eq!(
            classify([(Change::Added, p("demos/Bass/X - y.mp3"))]),
            Bump::Patch
        );
        assert_eq!(
            classify([(Change::Modified, p("taxonomy.toml"))]),
            Bump::Patch
        );
        assert_eq!(classify([(Change::Modified, p("README.md"))]), Bump::None);
        assert_eq!(
            classify([(Change::Modified, p("crates/deepmind-patches/src/lib.rs"))]),
            Bump::None
        );
        assert_eq!(
            classify([
                (Change::Modified, p("docs/format.md")),
                (Change::Added, p("presets/Pad/A - b.syx"))
            ]),
            Bump::Release
        );
    }
}
