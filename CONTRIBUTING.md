# Contributing

## Add a patch

1. Save the sound on the instrument or from your editor.
2. Name it on the instrument. Those sixteen characters become the filename.
3. Set its category on the instrument. That decides the folder: the
   instrument's `SFX` is `presets/Sound Effects/`, `Perc` is
   `presets/Percussion/`. The table in [README.md](README.md) maps all twelve.
4. Put the `.syx` in that folder, or in one collection folder inside it. Name
   it `Name - author.syx`.
5. Write `Name - author.toml` beside it. Fields are in
   [docs/format.md](docs/format.md); the minimum is below.
6. Optionally record up to 20 seconds of it and put the MP3 under `demos/` in
   the same tree. Format in [demos/README.md](demos/README.md).
7. Open a pull request. CI names each problem by file and line.

The minimum `.toml`:

```toml
name    = "Acid Growl"   # exactly the name stored in the program
author  = "nyx"
version = 1
about   = "Resonant 303-ish bass that opens under velocity."
licence = "CC-BY-4.0"
role    = ["bass"]       # at least one term from taxonomy.toml on any axis
```

`cargo run -p validate -- new --category Bass --name "Acid Growl" --author nyx --from dump.syx`
writes both files for you, with the program stored as A1.

## Three things to know first

- **A patch is not a trademark.** Sounds named after a record, a film or
  another manufacturer's instrument get renamed at review.
- **Only contribute what is yours to give.** A patch lifted from a commercial
  soundbank is not.
- **The licence field is yours.** It is printed as written and never
  interpreted. `CC-BY-4.0` is the suggested default; any SPDX identifier or
  the exact text `All rights reserved` is accepted.

## Update a patch

Edit the files in place. If the `.syx` bytes change, bump `version` by one. If
only the `.toml` changes, leave `version` alone. CI checks both.

## What CI checks

Run it yourself with `cargo run -p validate -- check`. Every rule:

1. Every `.syx` has a `.toml` beside it, and the reverse.
2. The `.toml` parses and has `name`, `author`, `version`, `about`, `licence`
   and at least one vocabulary term.
3. Every vocabulary term is in `taxonomy.toml`.
4. The `.syx` holds exactly one program dump.
5. It is stored as bank A, program 1. `validate normalise` rewrites it.
6. Its category byte is one of the twelve and matches its folder.
7. Its stored name equals the `.toml` name.
8. The filename stem equals `Name - author`, cleaned of `/ \ : * ? " < > |`.
9. No two patches in one folder share a name and an author.
10. A patch sits in its category folder or one collection folder inside it.
11. `licence` is an SPDX identifier or `All rights reserved`.
12. Every declared demo exists, every demo file is declared, and each is MP3,
    128 kbit/s constant, 44.1 kHz, stereo, at most 20 seconds and 350 KB.
13. Nothing sits outside `presets/`, `demos/`, `resources/`, the tooling
    folders and the root documents.
14. Icons are seven rows of seven `#` or `.` characters, and not blank. Every
    category and every vocabulary term has one under `resources/icons/`.
15. `version` is bumped when the `.syx` bytes changed since `main`.
16. Unknown `.toml` fields are refused. Anything the `.syx` already says is
    derived, never typed.

Missing `model` is a warning, not an error. Say what the patch was voiced on
if you can: a unison patch behaves differently on a DeepMind 6.

## Propose a vocabulary term

Add it to `taxonomy.toml` with a one-line description, and its 7x7 icon under
`resources/icons/<axis>/<term>.toml`, in its own pull request. Terms are
lowercase words. Then run `cargo run -p validate -- icons --write docs/icons.md`
so the preview stays current.

## Tooling changes

`crates/` and `tools/` are Rust. `cargo fmt`, `cargo clippy --workspace
--all-targets --all-features` and `cargo test --workspace` must pass. TOML is
formatted with `taplo fmt`. Changes there need a maintainer review; patch
contributions need only green CI.
