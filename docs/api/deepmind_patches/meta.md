**deepmind_patches > meta**

# Module: meta

## Contents

**Structs**

- [`Demo`](#demo) - One audio demo of a patch. The file name is computed, never typed.
- [`PatchMeta`](#patchmeta) - Everything a patch `.toml` may hold.

**Constants**

- [`ALL_RIGHTS_RESERVED`](#all_rights_reserved) - The exact text accepted in `licence` for a patch its author keeps all
- [`MAX_ABOUT_CHARS`](#max_about_chars) - Longest `about` may be, in characters. Two sentences, not an essay.
- [`MAX_DEMOS`](#max_demos) - Most demos a patch may declare.
- [`MAX_VARIANT_CHARS`](#max_variant_chars) - Longest a demo variant label may be, in characters.

---

## deepmind_patches::meta::ALL_RIGHTS_RESERVED

*Constant*: `&str`

The exact text accepted in `licence` for a patch its author keeps all
rights to.



## deepmind_patches::meta::Demo

*Struct*

One audio demo of a patch. The file name is computed, never typed.

**Fields:**
- `variant: Option<String>` - What is different about this take: `mod wheel`, `aftertouch`, `arp`.
- `about: Option<String>` - One line on how it was played.

**Traits:** Eq

**Trait Implementations:**

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
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



## deepmind_patches::meta::MAX_ABOUT_CHARS

*Constant*: `usize`

Longest `about` may be, in characters. Two sentences, not an essay.



## deepmind_patches::meta::MAX_DEMOS

*Constant*: `usize`

Most demos a patch may declare.



## deepmind_patches::meta::MAX_VARIANT_CHARS

*Constant*: `usize`

Longest a demo variant label may be, in characters.



## deepmind_patches::meta::PatchMeta

*Struct*

Everything a patch `.toml` may hold.

**Fields:**
- `name: String` - The program's stored name, sixteen characters at most.
- `author: String` - A person or a handle. Not an email address.
- `version: u32` - Bumped whenever the `.syx` bytes change. Starts at 1.
- `about: String` - What it sounds like and how to play it.
- `licence: String` - SPDX identifier, or [`ALL_RIGHTS_RESERVED`].
- `genre: Vec<String>` - Terms from `taxonomy.toml`.
- `mood: Vec<String>` - Terms from `taxonomy.toml`.
- `timbre: Vec<String>` - Terms from `taxonomy.toml`.
- `role: Vec<String>` - Terms from `taxonomy.toml`.
- `icon: Option<crate::icon::Icon>` - The patch's own icon. Falls back to the category icon when absent.
- `created: Option<toml::value::Datetime>` - When it was made. A TOML date.
- `model: Option<String>` - Which instrument it was voiced on, as Behringer writes it.
- `firmware: Option<String>` - Firmware it was made and checked on.
- `source: Option<String>` - Where it came from, if anywhere.
- `tags: Vec<String>` - Free text, searched but never styled.
- `demos: Vec<Demo>` - Audio demos, first is the default.
- `unknown: toml::Table` - Fields this crate does not know. Empty in a valid file; kept so an older

**Methods:**

- `fn parse(text: &str) -> std::result::Result<Self, String>` - Parses TOML text. Unknown fields land in [`PatchMeta::unknown`].
- `fn read(path: &Path) -> Result<Self>` - Reads and parses a file.
- `fn to_toml(self: &Self) -> Result<String>` - Serialises to TOML in the layout the repository uses.
- `fn stem(self: &Self) -> String` - The filename stem both files share.
- `fn demo_file_name(self: &Self, demo: &Demo) -> String` - The file name of one demo, without a folder.
- `fn terms(self: &Self, axis: Axis) -> &[String]` - The terms on one axis.
- `fn has_terms(self: &Self) -> bool` - Whether at least one vocabulary term is set on any axis.
- `fn licence_is_valid(self: &Self) -> bool` - Whether the licence field is an SPDX identifier or the accepted

**Trait Implementations:**

- **Serialize**
  - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private229::Result<<__S as >::Ok, <__S as >::Error>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, <__D as >::Error>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



