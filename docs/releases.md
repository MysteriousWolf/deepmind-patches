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

There is nothing to cut. The version is `package.version` in
`crates/deepmind-patches/Cargo.toml`, every pull request that ships something
moves it, and the **Release** workflow runs on every push to `main`:

1. Read the version. If `v<version>` is already tagged, stop.
2. Validate, build the index and the archives, write notes listing the
   patches added and updated since the previous tag.
3. Publish the GitHub release under the new tag.
4. Publish the crate at the same version, unless crates.io already has it.

Nothing is committed back. The tag is the record. The workflow can be run by
hand to retry a failed publish.

## The crate

The crate shares the version. CI holds every pull request to it:
`validate versioning` fails when something that ships changed and the version
did not, when the bump is smaller than the change needs, or when the new
version is already tagged or on crates.io. For code, the step means:

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
