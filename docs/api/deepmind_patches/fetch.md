**deepmind_patches > fetch**

# Module: fetch

## Contents

**Functions**

- [`asset_url`](#asset_url) - URL of one release's asset by tag.
- [`demos_url`](#demos_url) - URL of the newest release's `demos.tar.gz`, which is `demos/`.
- [`entries`](#entries) - Reads a `.tar.gz` already in memory, yielding `(path, bytes)` per file.
- [`file`](#file) - Downloads one `.syx` (or any file) pinned to the commit an index was built
- [`get`](#get) - Downloads a URL to memory.
- [`index_url`](#index_url) - URL of the newest release's `index.toml`.
- [`latest_index`](#latest_index) - Downloads and parses the newest release's index.
- [`patches_url`](#patches_url) - URL of the newest release's `patches.tar.gz`, which is `presets/`.
- [`unpack`](#unpack) - Downloads a `.tar.gz` asset and unpacks it into a folder.
- [`update_for`](#update_for) - The newest index when it is newer than the one held, else `None`.

**Constants**

- [`DEFAULT_REPO`](#default_repo) - The repository releases come from, as `owner/name`.

---

## deepmind_patches::fetch::DEFAULT_REPO

*Constant*: `&str`

The repository releases come from, as `owner/name`.



## deepmind_patches::fetch::asset_url

*Function*

URL of one release's asset by tag.

```rust
fn asset_url(owner_repo: &str, version: crate::version::LibraryVersion, asset: &str) -> String
```



## deepmind_patches::fetch::demos_url

*Function*

URL of the newest release's `demos.tar.gz`, which is `demos/`.

```rust
fn demos_url(owner_repo: &str) -> String
```



## deepmind_patches::fetch::entries

*Function*

Reads a `.tar.gz` already in memory, yielding `(path, bytes)` per file.

```rust
fn entries(archive: &[u8]) -> crate::error::Result<Vec<(String, Vec<u8>)>>
```



## deepmind_patches::fetch::file

*Function*

Downloads one `.syx` (or any file) pinned to the commit an index was built
from.

```rust
fn file(index: &crate::index::Index, owner_repo: &str, file: &str) -> crate::error::Result<Vec<u8>>
```



## deepmind_patches::fetch::get

*Function*

Downloads a URL to memory.

```rust
fn get(url: &str) -> crate::error::Result<Vec<u8>>
```



## deepmind_patches::fetch::index_url

*Function*

URL of the newest release's `index.toml`.

```rust
fn index_url(owner_repo: &str) -> String
```



## deepmind_patches::fetch::latest_index

*Function*

Downloads and parses the newest release's index.

```rust
fn latest_index(owner_repo: &str) -> crate::error::Result<crate::index::Index>
```



## deepmind_patches::fetch::patches_url

*Function*

URL of the newest release's `patches.tar.gz`, which is `presets/`.

```rust
fn patches_url(owner_repo: &str) -> String
```



## deepmind_patches::fetch::unpack

*Function*

Downloads a `.tar.gz` asset and unpacks it into a folder.

`patches.tar.gz` unpacks to `<dest>/presets/...`, `demos.tar.gz` to
`<dest>/demos/...`.

```rust
fn unpack(url: &str, dest: &std::path::Path) -> crate::error::Result<()>
```



## deepmind_patches::fetch::update_for

*Function*

The newest index when it is newer than the one held, else `None`.

```rust
fn update_for(held: &crate::index::Index, owner_repo: &str) -> crate::error::Result<Option<crate::index::Index>>
```



