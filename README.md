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

Shared patches for Behringer DeepMind 6, 12 and 12D synthesizers. One `.syx`
per sound, sorted by category. Download one and load it with any librarian.

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

Every `.syx` is stored as bank A, program 1; your librarian asks where to put
it. Audio previews live under [demos/](demos) in the same tree. Everything is
bundled in each [release](https://github.com/MysteriousWolf/deepmind-patches/releases/latest).

## For artists

A patch is two files with the same name, in the folder of its category:

```
presets/Bass/Acid Growl - nyx.syx     the program, saved from the instrument
presets/Bass/Acid Growl - nyx.toml    what you say about it
demos/Bass/Acid Growl - nyx.mp3       optional, up to 20 seconds of it alone
```

The name is the one stored in the program. The category is the one set on the
instrument. The `.toml` needs five lines and at least one term from
[taxonomy.toml](taxonomy.toml):

```toml
name    = "Acid Growl"
author  = "nyx"
version = 1                    # bump when the sound changes
about   = "Resonant 303-ish bass that opens under velocity."
licence = "CC-BY-4.0"          # yours to choose
role    = ["bass"]
```

Add an `icon` of seven rows of `#` and `.` if you want the patch to carry its
own 7x7 picture. Open a pull request and CI tells you what to fix.
[CONTRIBUTING.md](CONTRIBUTING.md) has the full field list and the rules.

## For developers

The [`deepmind-patches`](https://crates.io/crates/deepmind-patches) crate reads
a checkout or a release, validates, fingerprints, and matches a program dumped
from an instrument back to its patch and version. API reference in
[docs/api](docs/api/index.md); format, versioning and release details in
[docs/](docs).

## Licence

The repository's own files are Apache-2.0. Each patch carries its own licence,
chosen by its author and printed as written.
