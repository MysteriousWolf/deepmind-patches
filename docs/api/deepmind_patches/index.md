**deepmind_patches > index**

# Module: index

## Contents

**Structs**

- [`Catalogue`](#catalogue) - What the index was built from.
- [`HistoryEntry`](#historyentry) - One version a patch has had, and the sound it was.
- [`Index`](#index) - The whole file.
- [`IndexDemo`](#indexdemo) - One demo, with the file resolved.
- [`IndexPatch`](#indexpatch) - One patch as the index describes it.
- [`Match`](#match) - What a fingerprint lookup found.

**Functions**

- [`raw_url`](#raw_url) - `https://raw.githubusercontent.com/<owner/repo>/<commit>/<file>`, with the

---

## deepmind_patches::index::Catalogue

*Struct*

What the index was built from.

**Fields:**
- `schema: u32` - Layout of this file. A reader refuses a number above [`INDEX_SCHEMA`].
- `version: crate::version::LibraryVersion` - The library version this index describes.
- `commit: String` - The commit every `file` path is valid at.
- `built: String` - When it was built, RFC 3339 in UTC.
- `patches: usize` - How many patches follow.

**Traits:** Eq

**Trait Implementations:**

- **Serialize**
  - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private229::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, <__D as >::Error>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



## deepmind_patches::index::HistoryEntry

*Struct*

One version a patch has had, and the sound it was.

**Fields:**
- `version: u32` - The `version` field at that time.
- `fingerprint: crate::fingerprint::Fingerprint` - The fingerprint at that version.

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Serialize**
  - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private229::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, <__D as >::Error>`



## deepmind_patches::index::Index

*Struct*

The whole file.

**Fields:**
- `catalogue: Catalogue` - Provenance.
- `taxonomy: crate::taxonomy::Taxonomy` - The vocabularies, so a reader needs no second file.
- `icons: crate::library::Icons` - Category and badge icons.
- `palette: crate::library::Palette` - Named colours, if the checkout has them.
- `patches: Vec<IndexPatch>` - Every patch, sorted by id.

**Methods:**

- `fn build(library: &Library, version: LibraryVersion, commit: &str, built: &str, history: &BTreeMap<String, Vec<HistoryEntry>>) -> Self` - Builds the index from a valid library.
- `fn parse(text: &str) -> Result<Self>` - Parses an index, refusing a schema this crate does not read.
- `fn to_toml(self: &Self) -> Result<String>` - Serialises the index.
- `fn get(self: &Self, id: &str) -> Option<&IndexPatch>` - One patch by id.
- `fn lookup(self: &Self, fingerprint: &Fingerprint) -> Option<Match>` - The patch and version a fingerprint belongs to, if any version of any
- `fn by_name<'a>(self: &'a Self, name: &'a str) -> impl Trait` - Patches whose stored name equals `name`. What to try when a fingerprint
- `fn in_category(self: &Self, category: Category) -> impl Trait` - The patches in one category.
- `fn raw_url(self: &Self, owner_repo: &str, file: &str) -> String` - The URL of one file at the commit this index was built from.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, <__D as >::Error>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Serialize**
  - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private229::Result<<__S as >::Ok, <__S as >::Error>`



## deepmind_patches::index::IndexDemo

*Struct*

One demo, with the file resolved.

**Fields:**
- `file: String` - Path relative to the repository root.
- `variant: Option<String>` - The variant label, absent on the default demo.
- `about: Option<String>` - How it was played.

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Serialize**
  - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private229::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, <__D as >::Error>`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## deepmind_patches::index::IndexPatch

*Struct*

One patch as the index describes it.

**Fields:**
- `id: String` - `bass/acid-growl-nyx`.
- `name: String` - The stored name.
- `author: String` - Who made it.
- `version: u32` - Current version of the sound.
- `about: String` - What it sounds like.
- `category: crate::category::Category` - The folder and the program byte.
- `collection: Option<String>` - The collection folder, if any.
- `file: String` - The `.syx` path relative to the repository root.
- `sha256: String` - SHA-256 of the file, hex.
- `fingerprint: crate::fingerprint::Fingerprint` - Identity of the sound at this version.
- `licence: String` - The licence text the author wrote.
- `genre: Vec<String>` - Vocabulary terms.
- `mood: Vec<String>` - Vocabulary terms.
- `timbre: Vec<String>` - Vocabulary terms.
- `role: Vec<String>` - Vocabulary terms.
- `tags: Vec<String>` - Free text.
- `icon: crate::icon::Icon` - The icon to draw: the patch's own or its category's.
- `own_icon: bool` - Whether `icon` is the patch's own rather than the category fallback.
- `created: Option<toml::value::Datetime>` - When it was made.
- `model: Option<String>` - Which instrument it was voiced on.
- `firmware: Option<String>` - Firmware it was checked on.
- `source: Option<String>` - Where it came from.
- `demos: Vec<IndexDemo>` - Demos, default first.
- `fx_mode: String` - `Insert`, `Send` or `Bypass`.
- `effects: Vec<String>` - Effect algorithms in engine order, empty when bypassed.
- `arp: bool` - Arpeggiator on.
- `arp_mode: Option<String>` - Arpeggiator mode, when on.
- `polyphony: String` - Polyphony mode label.
- `unison: u8` - Voices per note.
- `routings: u8` - Modulation routings in use.
- `sequencer: bool` - Control sequencer enabled.
- `transpose: i8` - Transpose in semitones.
- `history: Vec<HistoryEntry>` - Every version this patch has had, oldest first, current last.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, <__D as >::Error>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Serialize**
  - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private229::Result<<__S as >::Ok, <__S as >::Error>`



## deepmind_patches::index::Match

*Struct*

What a fingerprint lookup found.

**Generic Parameters:**
- 'a

**Fields:**
- `patch: &'a IndexPatch` - The patch.
- `version: u32` - The version whose fingerprint matched.
- `latest: bool` - Whether that is the patch's current version.

**Traits:** Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## deepmind_patches::index::raw_url

*Function*

`https://raw.githubusercontent.com/<owner/repo>/<commit>/<file>`, with the
path percent-encoded.

```rust
fn raw_url(owner_repo: &str, commit: &str, file: &str) -> String
```



