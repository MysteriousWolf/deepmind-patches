# Releases

## Assets

| Asset | Contents |
| --- | --- |
| `index.toml` | Every patch: metadata, derived facts, resolved icon, demos, version history. Plus the taxonomy, the icons and the palette. |
| `patches.tar.gz` | The `presets/` folder. |
| `demos.tar.gz` | The `demos/` folder. Optional download. |

The logo and banner in `docs/` are the repository's own mark, drawn in the same case as deepmind-midi's and deepmind-control's.

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

## The crate

The release workflow also publishes `crates/deepmind-patches` when the
version in its `Cargo.toml` is not on crates.io yet. Bump it on the same
`YY.RELEASE.PATCH` scheme, on its own counter, in the pull request that changes
the code. A release that changed no code publishes nothing.

CI holds the pull request to that. `validate crate-version` fails when
`crates/deepmind-patches/` changed and the version did not, or when the new
version is not exactly one step from `main`:

| Step | For |
| --- | --- |
| `PATCH` + 1 | A fix. Nothing public added or changed. |
| `RELEASE` + 1, `PATCH` = 0 | Something public added. |
| `YY` = this year, others 0 | Something public removed or changed, or the first code change of a year. |

Cargo reads the three parts as major, minor and patch, so a host that depends
on `26.1` is offered every later `26.x` automatically. That is why a breaking
change needs the year part, even mid-year: `cargo semver-checks` compares the
public API with `main` and fails a bump that is smaller than the change. CI
also refuses a version that is already on crates.io.

## API reference

`docs/api/` is Markdown generated from the crate's rustdoc by
`tools/api-docs.sh`. CI checks it is current; run the script after changing
public items.
