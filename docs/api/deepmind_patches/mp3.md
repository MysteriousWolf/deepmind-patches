**deepmind_patches > mp3**

# Module: mp3

## Contents

**Modules**

- [`limits`](#limits) - The demo format the repository accepts.

**Structs**

- [`Mp3Info`](#mp3info) - What the frame headers of a file say.

---

## deepmind_patches::mp3::Mp3Info

*Struct*

What the frame headers of a file say.

**Fields:**
- `seconds: f64` - Playing time in seconds.
- `bitrate_kbps: u32` - Bitrate of the first frame, kbit/s.
- `constant_bitrate: bool` - Whether every frame has the same bitrate.
- `sample_rate: u32` - Sample rate, Hz.
- `channels: u8` - 1 or 2.
- `frames: u32` - Frames counted.

**Methods:**

- `fn parse(path: &Path, bytes: &[u8]) -> Result<Self>` - Reads the frame headers of a file's bytes.
- `fn read(path: &Path) -> Result<Self>` - Reads a file.

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## Module: limits

The demo format the repository accepts.



