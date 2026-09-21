**deepmind_patches > category**

# Module: category

## Contents

**Enums**

- [`Category`](#category) - One of the twelve categories, in the instrument's order.

---

## deepmind_patches::category::Category

*Enum*

One of the twelve categories, in the instrument's order.

**Variants:**
- `Bass` - Plays the bottom.
- `Pad` - Held chords.
- `Lead` - Plays the tune.
- `Mono` - Monophonic, not otherwise a bass or a lead.
- `Poly` - Polyphonic, not otherwise a pad.
- `Stab` - One hit, chordal.
- `Sfx` - Sound effects.
- `Arp` - The arpeggiator is the point.
- `Seq` - The sequencer is the point.
- `Perc` - Drums, hits, clicks.
- `Ambient` - Drifting, atmospheric.
- `Modular` - Patches built around the modulation matrix.

**Methods:**

- `fn from_value(value: u8) -> Option<Self>` - The category a stored value names, or `None` for `None` and `User-1`
- `fn value(self: Self) -> u8` - The value the program stores for this category.
- `fn name(self: Self) -> &'static str` - The readable name: the folder under `presets/`, the icon file, and
- `fn label(self: Self) -> &'static str` - The short label the instrument's display uses: `SFX`, `Perc`.
- `fn about(self: Self) -> &'static str` - What the category is for, in a few words.
- `fn from_name(name: &str) -> Option<Self>` - The category a folder name refers to. Case-sensitive.
- `fn from_label(label: &str) -> Option<Self>` - The category an instrument label refers to. Case-sensitive.
- `fn of(program: &Program) -> Option<Self>` - The category a program is stored with.
- `fn label_of_value(value: u8) -> String` - The label the instrument shows for a stored value, including the ones

**Traits:** Eq, Copy

**Trait Implementations:**

- **FromStr**
  - `fn from_str(name: &str) -> std::result::Result<Self, <Self as >::Err>`
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, <__D as >::Error>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Serialize**
  - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private229::Result<<__S as >::Ok, <__S as >::Error>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> $crate::cmp::Ordering`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> $crate::option::Option<$crate::cmp::Ordering>`



