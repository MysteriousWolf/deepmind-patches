# Releases

## Assets

| Asset | Contents |
| --- | --- |
| `index.toml` | Every patch: metadata, derived facts, resolved icon, demos, version history. Plus the taxonomy, the icons and the palette. |
| `patches.tar.gz` | The `presets/` folder. |
| `demos.tar.gz` | The `demos/` folder. Optional download. |

URLs, none of which touch the GitHub API or need a token:

| | |
| --- | --- |
| Newest index | `https://github.com/MysteriousWolf/deepmind-patches/releases/latest/download/index.toml` |
| Newest patches | `.../releases/latest/download/patches.tar.gz` |
| Newest demos | `.../releases/latest/download/demos.tar.gz` |
| A specific release | `.../releases/download/v26.1.0/index.toml` |
| One file, pinned | `https://raw.githubusercontent.com/MysteriousWolf/deepmind-patches/<commit>/<file>` |

`commit` in the index is the revision every `file` path is valid at. A host
holding last week's index fetches last week's files, never a mixture.

## Cutting a release

`main` is always releasable: every merge passed the validator. A release is a
manual step so several pull requests can batch into one:

1. Run the **Release** workflow. Leave the bump on `auto`, or force `patch`,
   `release` or `year`.
2. The workflow validates, computes the next version from the last tag and
   the changes since, builds the index and the archives, writes notes listing
   added and updated patches, and publishes the release under the new tag.

Nothing is committed back. The tag is the record.

## Publishing the crate

The **Publish crate** workflow runs `cargo publish` for
`crates/deepmind-patches`. Bump its version in `Cargo.toml` first, on the same
`YY.RELEASE.PATCH` scheme, on its own counter.
