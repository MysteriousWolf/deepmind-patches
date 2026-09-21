#!/bin/sh
# Regenerates docs/api/ from the crate's rustdoc, as Markdown.
#
# rustdoc's JSON output is nightly-only and changes between nightlies, so the
# toolchain and the converter are pinned. Bump both together and regenerate.
# CI runs this and fails if docs/api/ differs from what is committed.
set -eu

NIGHTLY="nightly-2026-09-21"
DOC_MD="0.11.0"

cd "$(dirname "$0")/.."

if ! rustup run "$NIGHTLY" rustc --version >/dev/null 2>&1; then
  rustup toolchain install "$NIGHTLY" --profile minimal
fi
if ! cargo install --list | grep -q "^cargo-doc-md v$DOC_MD:"; then
  cargo install cargo-doc-md --version "$DOC_MD" --locked
fi

cargo "+$NIGHTLY" rustdoc -p deepmind-patches --all-features -- \
  -Z unstable-options --output-format json
rm -rf docs/api
cargo doc-md --json target/doc/deepmind_patches.json -o docs/api
