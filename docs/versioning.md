# Versioning

## The version

There is one version, `package.version` in `crates/deepmind-patches/Cargo.toml`.
It is the library's, the crate's and the release tag's: `26.3.1` in the file
is `v26.3.1` on GitHub and `26.3.1` on crates.io.

| Part | Bumps when |
| --- | --- |
| `YY` | The first release of a year, from the date. Resets the other two to 0. Also a breaking change to the crate's API. |
| `RELEASE` | New patches were added, or something public was added to the crate. Resets `PATCH` to 0. |
| `PATCH` | Existing patches, demos, taxonomy, resources or crate code changed without adding anything. |

Every pull request that changes something that ships bumps the version, in
the same pull request. CI works out the least bump the change needs from the
paths it touches, refuses a smaller one, and refuses a version that is already
tagged or on crates.io. Bumping the version changes `Cargo.lock` too; run
`cargo check` after editing it.

Workflows, docs and the tool under `tools/` need no bump. Merging to `main`
releases whatever version is not tagged yet.

## A patch

Each `.toml` carries an integer `version`, starting at 1. It bumps by one
whenever the `.syx` bytes change and stays put when only the `.toml` changes.
CI compares each changed patch with `main` and refuses a changed `.syx` without
a bump.

## Fingerprints

A fingerprint is the SHA-256 of the program's parameter bytes with the name and
the category left out. The bank, the slot and the sysex framing are not part of
the program, and renaming or recategorising on the front panel does not make it
a different sound. Any other edit does.

The index carries the fingerprint of every version each patch has ever had.
A host that dumps a program from an instrument fingerprints it and looks it
up:

- Exact match: the patch, the version, and whether a newer version exists.
- No match, but the stored name equals a patch name: based on that patch,
  edited locally.
- Nothing: unknown.

The history is derived from git at release time, one entry per version, so
nobody maintains it and it cannot drift.
