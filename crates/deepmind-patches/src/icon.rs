//! The 7x7 one-bit icon.
//!
//! Written in TOML as seven strings of seven characters, `#` lit and `.` unlit,
//! so the picture is readable in a diff:
//!
//! ```toml
//! icon = [
//!   ".......",
//!   "..###..",
//!   ".#...#.",
//!   ".#...#.",
//!   ".#####.",
//!   ".#...#.",
//!   ".......",
//! ]
//! ```
//!
//! The bit layout matches [`deepmind_midi::pixels::Pixels`], so a host that
//! already draws the instrument's own glyphs draws these the same way. This
//! crate never renders; it hands out bits.

use std::fmt;

use deepmind_midi::pixels::{Pixels, SIDE};
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::error::{Error, Result};

/// Character for a lit pixel.
pub const LIT: char = '#';
/// Character for an unlit pixel.
pub const UNLIT: char = '.';

/// A 7x7 one-bit picture. Row 0 is the top; bit 0 of a row is the left.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Icon {
    rows: [u8; SIDE],
}

impl Icon {
    /// Dots across and down.
    pub const SIDE: usize = SIDE;

    /// Builds an icon from packed rows, bit 0 leftmost.
    #[must_use]
    pub const fn from_rows(rows: [u8; SIDE]) -> Self {
        Self { rows }
    }

    /// Parses the seven text rows the TOML carries.
    ///
    /// # Errors
    ///
    /// Anything but exactly seven rows of exactly seven `#` or `.` characters.
    pub fn parse<S: AsRef<str>>(rows: &[S]) -> Result<Self> {
        if rows.len() != SIDE {
            return Err(Error::Icon(format!(
                "expected {SIDE} rows, found {}",
                rows.len()
            )));
        }
        let mut packed = [0u8; SIDE];
        for (y, row) in rows.iter().enumerate() {
            let row = row.as_ref();
            if row.chars().count() != SIDE {
                return Err(Error::Icon(format!(
                    "row {} has {} characters, expected {SIDE}",
                    y + 1,
                    row.chars().count()
                )));
            }
            for (x, character) in row.chars().enumerate() {
                match character {
                    LIT => packed[y] |= 1 << x,
                    UNLIT => {}
                    other => {
                        return Err(Error::Icon(format!(
                            "row {} column {} is {other:?}, expected {LIT:?} or {UNLIT:?}",
                            y + 1,
                            x + 1
                        )));
                    }
                }
            }
        }
        Ok(Self { rows: packed })
    }

    /// The seven text rows, as the TOML writes them.
    #[must_use]
    pub fn rows_text(&self) -> [String; SIDE] {
        std::array::from_fn(|y| {
            (0..SIDE)
                .map(|x| if self.is_lit(x, y) { LIT } else { UNLIT })
                .collect()
        })
    }

    /// Packed rows, bit 0 leftmost.
    #[must_use]
    pub const fn rows(&self) -> &[u8; SIDE] {
        &self.rows
    }

    /// Whether the pixel at `x`, `y` is lit. `false` outside the grid.
    #[must_use]
    pub fn is_lit(&self, x: usize, y: usize) -> bool {
        x < SIDE && self.rows.get(y).is_some_and(|row| row & (1 << x) != 0)
    }

    /// Whether nothing is lit.
    #[must_use]
    pub fn is_blank(&self) -> bool {
        self.rows.iter().all(|row| *row == 0)
    }

    /// The same picture as the protocol crate's grid type.
    #[must_use]
    pub const fn pixels(&self) -> Pixels {
        Pixels::new(self.rows)
    }

    /// Builds an icon from the protocol crate's grid type.
    #[must_use]
    pub const fn from_pixels(pixels: &Pixels) -> Self {
        Self {
            rows: *pixels.rows(),
        }
    }
}

impl From<Pixels> for Icon {
    fn from(pixels: Pixels) -> Self {
        Self::from_pixels(&pixels)
    }
}

impl From<Icon> for Pixels {
    fn from(icon: Icon) -> Self {
        icon.pixels()
    }
}

impl fmt::Debug for Icon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.rows_text()).finish()
    }
}

impl fmt::Display for Icon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.rows_text() {
            writeln!(f, "{row}")?;
        }
        Ok(())
    }
}

impl Serialize for Icon {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        self.rows_text().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Icon {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let rows: Vec<String> = Vec::deserialize(deserializer)?;
        Self::parse(&rows).map_err(D::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const A: [&str; 7] = [
        ".......", "..###..", ".#...#.", ".#...#.", ".#####.", ".#...#.", ".......",
    ];

    #[test]
    fn parses_and_prints_the_same_rows() {
        let icon = Icon::parse(&A).unwrap();
        assert_eq!(
            icon.rows_text()
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            A
        );
        assert!(icon.is_lit(2, 1));
        assert!(!icon.is_lit(0, 0));
        assert!(!icon.is_lit(7, 0));
        assert!(!icon.is_blank());
        assert!(Icon::default().is_blank());
    }

    #[test]
    fn matches_the_protocol_crate_layout() {
        let icon = Icon::parse(&A).unwrap();
        let pixels = icon.pixels();
        for y in 0..7u8 {
            for x in 0..7u8 {
                assert_eq!(pixels.is_lit(x, y), icon.is_lit(x.into(), y.into()));
            }
        }
        assert_eq!(Icon::from_pixels(&pixels), icon);
    }

    #[test]
    fn refuses_bad_shapes() {
        assert!(Icon::parse(&A[..6]).is_err());
        assert!(Icon::parse(&["......", ".", ".", ".", ".", ".", "."]).is_err());
        assert!(Icon::parse(&["......x", A[1], A[2], A[3], A[4], A[5], A[6]]).is_err());
    }

    #[test]
    fn toml_round_trip() {
        #[derive(Serialize, Deserialize)]
        struct Holder {
            icon: Icon,
        }
        let holder = Holder {
            icon: Icon::parse(&A).unwrap(),
        };
        let text = toml::to_string(&holder).unwrap();
        let back: Holder = toml::from_str(&text).unwrap();
        assert_eq!(back.icon, holder.icon);
    }
}
