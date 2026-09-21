# Versioning

## The library

Releases are tagged `vYY.RELEASE.PATCH`, for example `v26.3.1`.

| Part | Bumps when |
| --- | --- |
| `YY` | The first release of a year. Resets the other two to 0. |
| `RELEASE` | New patches were added. Resets `PATCH` to 0. |
| `PATCH` | Only existing patches, demos, taxonomy or resources changed. |

The version lives in git tags, not in a file, so a contribution never edits a
shared version line and never conflicts with another one. CI classifies each
pull request as `release`, `patch` or `none` from the paths it touches, and the
release workflow computes the next tag from everything merged since the last
one. A maintainer can force a bump when cutting a release.

Tooling, workflows and docs need no bump. The `deepmind-patches` crate has its
own version on the same scheme, on its own counter.

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
