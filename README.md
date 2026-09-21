<p align="center">
  <img src="https://raw.githubusercontent.com/MysteriousWolf/deepmind-patches/main/docs/banner.svg" alt="deepmind-patches" width="800">
</p>

<p align="center">
  <a href="https://github.com/MysteriousWolf/deepmind-patches/releases/latest"><img src="https://img.shields.io/github/v/release/MysteriousWolf/deepmind-patches?label=library" alt="Latest release"></a>
  <a href="https://crates.io/crates/deepmind-patches"><img src="https://img.shields.io/crates/v/deepmind-patches" alt="crates.io"></a>
  <a href="https://github.com/MysteriousWolf/deepmind-patches/actions/workflows/pr.yml"><img src="https://github.com/MysteriousWolf/deepmind-patches/actions/workflows/pr.yml/badge.svg" alt="Check"></a>
  <a href="https://github.com/MysteriousWolf/deepmind-midi"><img src="https://img.shields.io/badge/built%20on-deepmind--midi-blue" alt="Built on deepmind-midi"></a>
  <a href="https://github.com/MysteriousWolf/deepmind-patches/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-Apache--2.0-green" alt="License: Apache-2.0"></a>
</p>

Shared patches for Behringer DeepMind 6, 12 and 12D synthesizers.

One `.syx` and one `.toml` per sound, sorted by category. Browse the folders,
download a `.syx`, load it with any librarian. No special software needed.

## Browse

| Folder | On the instrument | What is in it |
| --- | --- | --- |
| [Bass](presets/Bass) | Bass | Plays the bottom |
| [Pads](presets/Pads) | Pad | Held chords |
| [Leads](presets/Leads) | Lead | Plays the tune |
| [Monophonic](presets/Monophonic) | Mono | One voice, not a bass or a lead |
| [Polyphonic](presets/Polyphonic) | Poly | Many voices, not a pad |
| [Stabs](presets/Stabs) | Stab | One hit, chordal |
| [Sound Effects](presets/Sound%20Effects) | SFX | Not an instrument |
| [Arpeggios](presets/Arpeggios) | Arp | The arpeggiator is the point |
| [Sequences](presets/Sequences) | Seq | The sequencer is the point |
| [Percussion](presets/Percussion) | Perc | Drums, hits, clicks |
| [Ambient](presets/Ambient) | Ambient | Drifting, atmospheric |
| [Modular](presets/Modular) | Modular | Built around the modulation matrix |

These are the instrument's own twelve categories, under readable names. The
category is stored in the patch itself, so a file can only sit in the folder
its program says. Each patch sits directly in its category folder, or in one
collection folder inside it. Files are named `Name - author.syx` with the
`.toml` beside them describing the sound.

Every patch, category and vocabulary term has a 7x7 icon, drawn for a dot
matrix like the instrument's own display. See [docs/icons.md](docs/icons.md).

Every `.syx` is stored as bank A, program 1. Your librarian asks where to put
it.

Audio previews, where present, are under [demos/](demos) in the same tree.

## Download

Every release ships three assets:

| Asset | Contents |
| --- | --- |
| `index.toml` | Every patch with its metadata, derived facts, icon and version history |
| `patches.tar.gz` | The `presets/` folder |
| `demos.tar.gz` | The `demos/` folder, optional |

The newest is always at
`https://github.com/MysteriousWolf/deepmind-patches/releases/latest/download/<asset>`.

## Contribute

Save the patch, name it and set its category on the instrument, drop the
`.syx` into its folder, write the `.toml` beside it, open a pull request. CI
checks everything and tells you what to fix. Details in
[CONTRIBUTING.md](CONTRIBUTING.md).

## Versions

Releases are tagged `vYY.RELEASE.PATCH`: `release` bumps when patches are
added, `patch` when existing ones change. Each patch also carries its own
integer `version`, bumped whenever its bytes change. A program dumped from an
instrument can be matched back to the patch and version it came from. See
[docs/versioning.md](docs/versioning.md).

## For developers

The `deepmind-patches` crate reads a checkout or an `index.toml`, validates,
fingerprints, and, with the `fetch` feature, downloads releases and checks for
updates. `tools/validate` is the CI tool built on it. See
[docs/format.md](docs/format.md), [docs/index.md](docs/index.md) and
[docs/releases.md](docs/releases.md).

## Licence

The repository's own files (tooling, docs, taxonomy, icons) are Apache-2.0.
Each patch carries its own licence in its `.toml`, chosen by its author and
printed as written. Nothing here interprets it.
