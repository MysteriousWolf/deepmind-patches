# Changelog

Release notes are generated from the patches each release adds or changes and
attached to the release on GitHub. This file records changes to the format and
the tooling.

## Unreleased

- Repository layout, patch format, taxonomy, icons, validator and crate.
- One version for everything, in `crates/deepmind-patches/Cargo.toml`.
  `validate versioning` checks each pull request moved it by at least what
  the change needs; merging to `main` tags and publishes it.
- CI builds on the minimum Rust, tests on Linux, macOS and Windows, checks
  `Cargo.lock`, dependencies, licences, unused dependencies, spelling and the
  workflows, and compares the crate's public API with `main`.
