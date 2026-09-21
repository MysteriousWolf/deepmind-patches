**deepmind_patches > error**

# Module: error

## Contents

**Enums**

- [`Error`](#error) - Anything this crate can fail with.

**Type Aliases**

- [`Result`](#result) - `Result` with this crate's [`Error`].

---

## deepmind_patches::error::Error

*Enum*

Anything this crate can fail with.

**Variants:**
- `Io{ path: std::path::PathBuf, source: std::io::Error }` - Reading or writing a file failed.
- `Toml{ path: std::path::PathBuf, message: String }` - A TOML file did not parse.
- `ProgramCount{ path: std::path::PathBuf, found: usize }` - A `.syx` file did not hold exactly one program.
- `Midi(deepmind_midi::Error)` - The underlying protocol library refused something.
- `Icon(String)` - An icon row was not seven of `#` or `.`.
- `Version(String)` - A version string was not `yy.release.patch`.
- `Fingerprint(String)` - A fingerprint string was not 64 hex digits.
- `Schema{ found: u32, supported: u32 }` - An index carries a schema this crate does not read.
- `Mp3{ path: std::path::PathBuf, message: String }` - An MP3 file could not be read as one.
- `Invalid(usize)` - Validation found errors. The findings carry the detail.
- `Fetch{ url: String, message: String }` - A network operation failed (feature `fetch`).

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(source: deepmind_midi::Error) -> Self`
- **Error**
  - `fn source(self: &Self) -> ::core::option::Option<&dyn ::thiserror::__private20::Error>`
- **Display**
  - `fn fmt(self: &Self, __formatter: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`



## deepmind_patches::error::Result

*Type Alias*: `std::result::Result<T, Error>`

`Result` with this crate's [`Error`].



