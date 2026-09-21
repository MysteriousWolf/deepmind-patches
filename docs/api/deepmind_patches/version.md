**deepmind_patches > version**

# Module: version

## Contents

**Structs**

- [`LibraryVersion`](#libraryversion) - A library version.

**Enums**

- [`Bump`](#bump) - What kind of bump a change needs.
- [`Change`](#change) - How a changed path was changed.

**Functions**

- [`classify`](#classify) - The bump a set of changed paths requires.

---

## deepmind_patches::version::Bump

*Enum*

What kind of bump a change needs.

**Variants:**
- `None` - Nothing that ships changed.
- `Patch` - Existing content changed.
- `Release` - Content was added.
- `Year` - First release of a new year.

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, <__D as >::Error>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Serialize**
  - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private229::Result<<__S as >::Ok, <__S as >::Error>`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> $crate::cmp::Ordering`



## deepmind_patches::version::Change

*Enum*

How a changed path was changed.

**Variants:**
- `Added` - The path is new.
- `Modified` - The path exists on both sides and differs.
- `Deleted` - The path is gone.
- `Renamed` - The path moved. Counts as modified.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## deepmind_patches::version::LibraryVersion

*Struct*

A library version.

**Fields:**
- `year: u32` - Two-digit year, `26` for 2026.
- `release: u32` - Content releases this year.
- `patch: u32` - Patches since the last release.

**Methods:**

- `fn tag(self: &Self) -> String` - The tag a version is released under: `v26.1.0`.
- `fn from_tag(tag: &str) -> Result<Self, Error>` - Parses a tag, with or without the `v`.
- `fn next(self: Self, bump: Bump, year: u32) -> Self` - The next version after a bump, given the current two-digit year.
- `fn step_to(self: Self, next: Self, year: u32) -> Result<Bump, Error>` - The bump that takes `self` to `next`, if `next` is exactly one step

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Self`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> $crate::cmp::Ordering`
- **Serialize**
  - `fn serialize<S>(self: &Self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`
- **FromStr**
  - `fn from_str(text: &str) -> Result<Self, <Self as >::Err>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Deserialize**
  - `fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## deepmind_patches::version::classify

*Function*

The bump a set of changed paths requires.

A new `.syx` under `presets/` is a release. Anything else that ships, which
is everything under `presets/`, `demos/` and `resources/`, plus
`taxonomy.toml`, is a patch. Tooling, workflows and docs need no bump.

```rust
fn classify<'a, impl IntoIterator<Item = (Change, &'a Path)>>(changes: impl Trait) -> Bump
```



