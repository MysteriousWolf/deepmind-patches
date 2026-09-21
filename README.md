# deepmind-patches

Shared patches for Behringer DeepMind 6, 12 and 12D synthesizers.

One `.syx` and one `.toml` per sound, sorted by category. Browse the folders,
download a `.syx`, load it with any librarian. No special software needed.

## Browse

| Folder | What is in it |
| --- | --- |
| [Bass](presets/Bass) | Plays the bottom |
| [Pad](presets/Pad) | Held chords |
| [Lead](presets/Lead) | Plays the tune |
| [Mono](presets/Mono) | Monophonic, not a bass or a lead |
| [Poly](presets/Poly) | Polyphonic, not a pad |
| [Stab](presets/Stab) | One hit, chordal |
| [SFX](presets/SFX) | Sound effects |
| [Arp](presets/Arp) | The arpeggiator is the point |
| [Seq](presets/Seq) | The sequencer is the point |
| [Perc](presets/Perc) | Drums, hits, clicks |
| [Ambient](presets/Ambient) | Drifting, atmospheric |
| [Modular](presets/Modular) | Built around the modulation matrix |

These are the instrument's own twelve categories. Each patch sits directly in
its category folder, or in one collection folder inside it. Files are named
`Name - author.syx` with the `.toml` beside them describing the sound.

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
