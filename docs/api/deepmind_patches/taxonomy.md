**deepmind_patches > taxonomy**

# Module: taxonomy

## Contents

**Structs**

- [`Taxonomy`](#taxonomy) - Term to one-line description, per axis.

**Enums**

- [`Axis`](#axis) - One of the four vocabulary axes.

**Functions**

- [`is_term`](#is_term) - Whether text is a valid term: lowercase ASCII letters, digits and `-`.

---

## deepmind_patches::taxonomy::Axis

*Enum*

One of the four vocabulary axes.

**Variants:**
- `Genre` - What music it is for.
- `Mood` - How it feels.
- `Timbre` - What it sounds like.
- `Role` - What it does in a track.

**Methods:**

- `fn name(self: Self) -> &'static str` - The key used in TOML.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Serialize**
  - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private229::Result<<__S as >::Ok, <__S as >::Error>`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> $crate::cmp::Ordering`
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



## deepmind_patches::taxonomy::Taxonomy

*Struct*

Term to one-line description, per axis.

**Fields:**
- `genre: std::collections::BTreeMap<String, String>` - What music it is for.
- `mood: std::collections::BTreeMap<String, String>` - How it feels.
- `timbre: std::collections::BTreeMap<String, String>` - What it sounds like.
- `role: std::collections::BTreeMap<String, String>` - What it does in a track.

**Methods:**

- `fn parse(text: &str) -> Result<Self>` - Parses `taxonomy.toml` text.
- `fn read(path: &Path) -> Result<Self>` - Reads and parses a file.
- `fn axis(self: &Self, axis: Axis) -> &BTreeMap<String, String>` - The terms on one axis.
- `fn contains(self: &Self, axis: Axis, term: &str) -> bool` - Whether a term exists on an axis.
- `fn malformed_terms(self: &Self) -> impl Trait` - Terms that are not lowercase ASCII words, which the file forbids.

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, <__D as >::Error>`
- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Serialize**
  - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private229::Result<<__S as >::Ok, <__S as >::Error>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## deepmind_patches::taxonomy::is_term

*Function*

Whether text is a valid term: lowercase ASCII letters, digits and `-`.

```rust
fn is_term(text: &str) -> bool
```



