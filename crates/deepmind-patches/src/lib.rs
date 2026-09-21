//! The shared `DeepMind` patch library, as code.
//!
//! One `.syx` and one `.toml` per sound, under `presets/<Category>/`. This crate
//! reads that layout, checks it, derives what the program bytes say, and builds
//! the `index.toml` a release ships. An application links it to browse a
//! checkout or an index, to match a program dumped from an instrument back to
//! the patch and version it came from, and to check for updates.
//!
//! | Module          | Responsibility                                            |
//! |-----------------|-----------------------------------------------------------|
//! | [`category`]    | The instrument's twelve categories, which are the folders |
//! | [`icon`]        | The 7x7 one-bit icon a patch or a category carries        |
//! | [`meta`]        | The patch `.toml`: what a person writes about a sound     |
//! | [`taxonomy`]    | The controlled vocabularies in `taxonomy.toml`            |
//! | [`derived`]     | What the 242 program bytes say, never typed by hand       |
//! | [`fingerprint`] | Identity of a sound, independent of name, slot and file   |
//! | [`library`]     | A checkout: every patch, demo and icon it holds           |
//! | [`validate`]    | Every rule CI enforces, as findings with a fix hint       |
//! | [`index`]       | The generated `index.toml`, built and read                 |
//! | [`version`]     | Library versioning: `yy.release.patch` and the bump rules |
//! | [`mp3`]         | Enough MP3 parsing to enforce the demo limits             |
//! | `fetch`         | Release download and update checks (feature `fetch`)      |
//!
//! ```no_run
//! use deepmind_patches::Library;
//!
//! let library = Library::open(".")?;
//! for patch in library.patches() {
//!     println!("{} by {} v{}", patch.meta.name, patch.meta.author, patch.meta.version);
//! }
//! # Ok::<(), deepmind_patches::Error>(())
//! ```

#![cfg_attr(test, allow(clippy::unwrap_used))]

pub mod category;
pub mod derived;
pub mod error;
#[cfg(feature = "fetch")]
pub mod fetch;
pub mod fingerprint;
pub mod icon;
pub mod index;
pub mod library;
pub mod meta;
pub mod mp3;
pub mod slug;
pub mod taxonomy;
pub mod validate;
pub mod version;

pub use category::Category;
pub use derived::Derived;
pub use error::{Error, Result};
pub use fingerprint::Fingerprint;
pub use icon::Icon;
pub use index::Index;
pub use library::{Library, Patch};
pub use meta::{Demo, PatchMeta};
pub use taxonomy::{Axis, Taxonomy};
pub use validate::{Finding, Severity};
pub use version::LibraryVersion;

/// Schema number of `index.toml` this crate reads and writes.
///
/// Bumped when a change would make an older reader misread the file. A reader
/// refuses a higher number instead of guessing.
pub const INDEX_SCHEMA: u32 = 1;

/// Folder the presets live in, relative to the repository root.
pub const PRESETS_DIR: &str = "presets";
/// Folder the demos live in, mirroring [`PRESETS_DIR`].
pub const DEMOS_DIR: &str = "demos";
/// Folder the icons and palette live in.
pub const RESOURCES_DIR: &str = "resources";
/// The vocabularies file at the root.
pub const TAXONOMY_FILE: &str = "taxonomy.toml";
