**deepmind_patches**

# Module: deepmind_patches

## Contents

**Modules**

- [`category`](#category) - The instrument's twelve program categories.
- [`derived`](#derived) - What the program bytes say. Computed, never typed.
- [`error`](#error) - The crate-wide error type.
- [`fetch`](#fetch) - Release download and update checks. Feature `fetch`.
- [`fingerprint`](#fingerprint) - The identity of a sound.
- [`icon`](#icon) - The 7x7 one-bit icon.
- [`index`](#index) - The generated `index.toml`.
- [`library`](#library) - A checkout of the repository: every patch, demo and icon it holds.
- [`meta`](#meta) - The patch `.toml`: what a person writes about a sound.
- [`mp3`](#mp3) - Enough MP3 parsing to enforce the demo limits.
- [`slug`](#slug) - Filenames computed from names, never the other way round.
- [`taxonomy`](#taxonomy) - The controlled vocabularies in `taxonomy.toml`.
- [`validate`](#validate) - Every rule the repository enforces, as findings that name the file and say
- [`version`](#version) - Library versioning: `yy.release.patch`.

**Constants**

- [`DEMOS_DIR`](#demos_dir) - Folder the demos live in, mirroring [`PRESETS_DIR`].
- [`INDEX_SCHEMA`](#index_schema) - Schema number of `index.toml` this crate reads and writes.
- [`PRESETS_DIR`](#presets_dir) - Folder the presets live in, relative to the repository root.
- [`RESOURCES_DIR`](#resources_dir) - Folder the icons and palette live in.
- [`TAXONOMY_FILE`](#taxonomy_file) - The vocabularies file at the root.

---

## deepmind_patches::DEMOS_DIR

*Constant*: `&str`

Folder the demos live in, mirroring [`PRESETS_DIR`].



## deepmind_patches::INDEX_SCHEMA

*Constant*: `u32`

Schema number of `index.toml` this crate reads and writes.

Bumped when a change would make an older reader misread the file. A reader
refuses a higher number instead of guessing.



## deepmind_patches::PRESETS_DIR

*Constant*: `&str`

Folder the presets live in, relative to the repository root.



## deepmind_patches::RESOURCES_DIR

*Constant*: `&str`

Folder the icons and palette live in.



## deepmind_patches::TAXONOMY_FILE

*Constant*: `&str`

The vocabularies file at the root.



## Module: category

The instrument's twelve program categories.

The category is byte 240 of the program, so it is the one classification
nobody types: the folder a patch sits in must agree with the file. `None`
and the four `User` slots are stored values but not categories here, and a
patch carrying one is refused until a category is set on the instrument.

Each category has two names. [`Category::name`] is the readable one, which
is the folder, the icon file and what the index writes: `Sound Effects`.
[`Category::label`] is the instrument's own short form: `SFX`.



## Module: derived

What the program bytes say. Computed, never typed.



## Module: error

The crate-wide error type.



## Module: fetch

Release download and update checks. Feature `fetch`.

Release assets are plain redirects to a CDN, not the GitHub API, so nothing
here needs a token or counts against the unauthenticated rate limit.



## Module: fingerprint

The identity of a sound.

A fingerprint is the SHA-256 of the program's parameter bytes with the name
and the category left out. A program dumped from an instrument after loading
matches the file it came from: bank, slot and sysex framing are not part of
the program, and renaming or recategorising on the front panel does not make
it a different sound. Any other edit does.

The index carries every fingerprint each patch version ever had, so a host
that dumps a program can name the patch and the version, and say whether a
newer one exists.



## Module: icon

The 7x7 one-bit icon.

Written in TOML as seven strings of seven characters, `#` lit and `.` unlit,
so the picture is readable in a diff:

```toml
icon = [
  ".......",
  "..###..",
  ".#...#.",
  ".#...#.",
  ".#####.",
  ".#...#.",
  ".......",
]
```

The bit layout matches [`deepmind_midi::pixels::Pixels`], so a host that
already draws the instrument's own glyphs draws these the same way. This
crate never renders; it hands out bits.



## Module: index

The generated `index.toml`.

Built by CI from a validated checkout and shipped with every release. An
application reads this one file to browse, search, match a dumped program
back to a patch and version, and fetch a single `.syx` pinned to the commit
the index was built from.

Nobody edits it by hand and nobody commits it.



## Module: library

A checkout of the repository: every patch, demo and icon it holds.

[`Library::scan`] reads everything and records what it could not read.
[`crate::validate`] turns a scan into findings. [`Library::open`] does both
and hands back typed patches only when nothing is wrong, which is what an
application wants; a validator wants the scan.



## Module: meta

The patch `.toml`: what a person writes about a sound.

Only what nothing else can know. Anything readable from the `.syx` is
derived by [`crate::derived`] and has no field here, because a typed fact
goes stale the moment someone edits the patch.

```toml
name    = "Acid Growl"          # equals the program's stored name
author  = "nyx"                 # a person or a handle
version = 1                     # bump whenever the .syx bytes change
about   = "Resonant 303-ish bass that opens under velocity."
licence = "CC-BY-4.0"           # SPDX identifier or "All rights reserved"

genre  = ["techno"]
mood   = ["dark", "aggressive"]
timbre = ["gritty", "analogue"]
role   = ["bass"]

icon = [".......", "..###..", ".#...#.", ".#...#.", ".#####.", ".#...#.", "......."]

created  = 2026-09-21
model    = "DeepMind 12"
firmware = "1.1.5"
source   = "Reworked from a factory program."
tags     = ["303", "velocity"]

[[demo]]
about = "Dry, C2 to C4."

[[demo]]
variant = "mod wheel"
about   = "Wheel fully up."
```



## Module: mp3

Enough MP3 parsing to enforce the demo limits.

Walks the frame headers, skipping an `ID3v2` tag at the front and an `ID3v1`
tag at the back. Reports duration, bitrate, sample rate and channels. Does
not decode audio.



## Module: slug

Filenames computed from names, never the other way round.

The stem of a patch is `{name} - {author}`. CI computes it from the TOML and
compares; it never parses a stem back into fields, so an author called
`Bits - Pieces` is fine.



## Module: taxonomy

The controlled vocabularies in `taxonomy.toml`.

Four axes, each answering a different question: `genre` is what music it is
for, `mood` is how it feels, `timbre` is what it sounds like, `role` is what
it does in a track. A term not in the file fails validation. Adding one is a
pull request.



## Module: validate

Every rule the repository enforces, as findings that name the file and say
what to do.

Rules are numbered so a contributor can find them in `CONTRIBUTING.md`.



## Module: version

Library versioning: `yy.release.patch`.

- `yy` is the two-digit year of the release. It resets the other two.
- `release` bumps when presets are added.
- `patch` bumps when only existing presets, demos, taxonomy or docs change.

The version lives in git tags (`v26.1.0`), never in a file, so a content
pull request never edits a shared version line and never conflicts with
another one. The release workflow computes the next version from what
changed since the last tag.

The crate that reads all this has its own version on the same scheme, on
its own counter, because code and content change at different times.



