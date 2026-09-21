**deepmind_patches > library**

# Module: library

## Contents

**Structs**

- [`IconFile`](#iconfile) - One icon file under `resources/icons/`.
- [`Icons`](#icons) - The icons a checkout carries: one per category, one per vocabulary term,
- [`Library`](#library) - A valid checkout.
- [`Patch`](#patch) - One valid patch.
- [`Scan`](#scan) - Everything read from a checkout, including what failed to read.
- [`Slot`](#slot) - One filename stem in one folder: the `.syx`, the `.toml`, and whatever

**Type Aliases**

- [`Palette`](#palette) - `resources/palette.toml`: named colours, grouped. Carried, not interpreted.

---

## deepmind_patches::library::IconFile

*Struct*

One icon file under `resources/icons/`.

**Fields:**
- `icon: crate::icon::Icon` - The picture.
- `about: Option<String>` - What it shows, one line.

**Traits:** Eq

**Trait Implementations:**

- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, <__D as >::Error>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Serialize**
  - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private229::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## deepmind_patches::library::Icons

*Struct*

The icons a checkout carries: one per category, one per vocabulary term,
plus badges. Each map is one folder under `resources/icons/`.

**Fields:**
- `categories: std::collections::BTreeMap<String, crate::icon::Icon>` - Keyed by category name. Complete in a valid checkout.
- `badges: std::collections::BTreeMap<String, crate::icon::Icon>` - Keyed by file stem: `arp`, `seq`, `unison`, `fx`.
- `genre: std::collections::BTreeMap<String, crate::icon::Icon>` - Keyed by genre term. Complete in a valid checkout.
- `mood: std::collections::BTreeMap<String, crate::icon::Icon>` - Keyed by mood term. Complete in a valid checkout.
- `timbre: std::collections::BTreeMap<String, crate::icon::Icon>` - Keyed by timbre term. Complete in a valid checkout.
- `role: std::collections::BTreeMap<String, crate::icon::Icon>` - Keyed by role term. Complete in a valid checkout.

**Methods:**

- `fn category(self: &Self, category: Category) -> Option<&Icon>` - The icon for a category, if present.
- `fn axis(self: &Self, axis: Axis) -> &BTreeMap<String, Icon>` - The icons for one vocabulary axis.
- `fn term(self: &Self, axis: Axis, term: &str) -> Option<&Icon>` - The icon for a vocabulary term, if present.
- `fn group(self: &Self, name: &str) -> Option<&BTreeMap<String, Icon>>` - One group by folder name.

**Traits:** Eq

**Trait Implementations:**

- **Serialize**
  - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private229::Result<<__S as >::Ok, <__S as >::Error>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, <__D as >::Error>`
- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## deepmind_patches::library::Library

*Struct*

A valid checkout.

**Fields:**
- `root: std::path::PathBuf` - The checkout root.
- `taxonomy: crate::taxonomy::Taxonomy` - The vocabularies.
- `icons: Icons` - The icons.
- `palette: Option<Palette>` - The palette, if any.

**Methods:**

- `fn scan(root: &Path) -> Result<Scan>` - Reads a checkout. Nothing is judged; see [`crate::validate`].
- `fn open<impl AsRef<Path>>(root: impl Trait) -> Result<Self>` - Reads and validates a checkout, refusing one with errors.
- `fn from_scan(scan: Scan) -> Self` - Builds typed patches out of a scan, keeping every slot that is whole.
- `fn patches(self: &Self) -> &[Patch]` - Every patch, sorted by id.
- `fn patch(self: &Self, id: &str) -> Option<&Patch>` - One patch by id.
- `fn in_category(self: &Self, category: Category) -> impl Trait` - The patches in one category.
- `fn by_fingerprint(self: &Self, fingerprint: &Fingerprint) -> Option<&Patch>` - The patch whose current bytes carry a fingerprint.
- `fn icon_for(self: &Self, patch: &Patch) -> Icon` - The icon to show for a patch: its own, else its category's, else blank.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## deepmind_patches::library::Palette

*Type Alias*: `std::collections::BTreeMap<String, std::collections::BTreeMap<String, String>>`

`resources/palette.toml`: named colours, grouped. Carried, not interpreted.



## deepmind_patches::library::Patch

*Struct*

One valid patch.

**Fields:**
- `id: String` - `bass/acid-growl-nyx`.
- `category: crate::category::Category` - The folder, which the program byte agrees with.
- `collection: Option<String>` - The collection folder, if any.
- `stem: String` - The filename stem.
- `syx_path: std::path::PathBuf` - The `.syx` relative to the root.
- `toml_path: std::path::PathBuf` - The `.toml` relative to the root.
- `meta: crate::meta::PatchMeta` - What the author wrote.
- `program: deepmind_midi::program::Program` - The program.
- `derived: crate::derived::Derived` - What the program says.
- `fingerprint: crate::fingerprint::Fingerprint` - Identity of the sound.
- `sha256: String` - SHA-256 of the `.syx` file, hex.
- `bytes: Vec<u8>` - The `.syx` bytes.

**Methods:**

- `fn demo_path(self: &Self, demo: &crate::meta::Demo) -> PathBuf` - The path a demo lives at, relative to the root.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## deepmind_patches::library::Scan

*Struct*

Everything read from a checkout, including what failed to read.

**Fields:**
- `root: std::path::PathBuf` - The checkout root.
- `taxonomy: Option<crate::taxonomy::Taxonomy>` - The vocabularies, if the file parsed.
- `icons: Icons` - The icons that parsed.
- `palette: Option<Palette>` - The palette, if present and parsed.
- `slots: Vec<Slot>` - Every stem under `presets/`, in path order.
- `demo_files: Vec<std::path::PathBuf>` - Every file under `demos/`, relative to the root.
- `stray: Vec<std::path::PathBuf>` - Every file under the root that no rule accounts for.
- `problems: Vec<crate::validate::Finding>` - Problems met while reading, before any rule ran.

**Methods:**

- `fn relative(self: &Self, path: &Path) -> PathBuf` - A path relative to the root, or the path itself if it is elsewhere.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## deepmind_patches::library::Slot

*Struct*

One filename stem in one folder: the `.syx`, the `.toml`, and whatever
reading them produced. What the validator looks at.

**Fields:**
- `folder: std::path::PathBuf` - Folder relative to the root: `presets/Pad/Aurora Pads`.
- `category_dir: String` - The first folder under `presets/`, which should be a category name.
- `collection: Option<String>` - The second folder, if any.
- `extra_depth: usize` - Folders below the category beyond the one allowed collection.
- `stem: String` - The filename without extension.
- `syx: Option<std::path::PathBuf>` - The `.syx` path relative to the root, if present.
- `toml: Option<std::path::PathBuf>` - The `.toml` path relative to the root, if present.
- `toml_text: Option<String>` - The `.toml` text, if it could be read.
- `meta: Option<crate::meta::PatchMeta>` - The parsed metadata, if it parsed.
- `bytes: Option<Vec<u8>>` - The `.syx` bytes, if they could be read.
- `programs: Vec<(Option<deepmind_midi::ids::Slot>, deepmind_midi::program::Program)>` - Every program the file carried, with the slot each names.

**Methods:**

- `fn program(self: &Self) -> Option<&Program>` - The one program, when the file holds exactly one.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



