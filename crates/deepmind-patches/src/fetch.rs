//! Release download and update checks. Feature `fetch`.
//!
//! Release assets are plain redirects to a CDN, not the GitHub API, so nothing
//! here needs a token or counts against the unauthenticated rate limit.

use std::io::Read;
use std::path::Path;

use crate::error::{Error, Result};
use crate::index::Index;
use crate::version::LibraryVersion;

/// The repository releases come from, as `owner/name`.
pub const DEFAULT_REPO: &str = "MysteriousWolf/deepmind-patches";

/// URL of the newest release's `index.toml`.
#[must_use]
pub fn index_url(owner_repo: &str) -> String {
    format!("https://github.com/{owner_repo}/releases/latest/download/index.toml")
}

/// URL of the newest release's `patches.tar.gz`, which is `presets/`.
#[must_use]
pub fn patches_url(owner_repo: &str) -> String {
    format!("https://github.com/{owner_repo}/releases/latest/download/patches.tar.gz")
}

/// URL of the newest release's `demos.tar.gz`, which is `demos/`.
#[must_use]
pub fn demos_url(owner_repo: &str) -> String {
    format!("https://github.com/{owner_repo}/releases/latest/download/demos.tar.gz")
}

/// URL of one release's asset by tag.
#[must_use]
pub fn asset_url(owner_repo: &str, version: LibraryVersion, asset: &str) -> String {
    format!(
        "https://github.com/{owner_repo}/releases/download/{}/{asset}",
        version.tag()
    )
}

/// Downloads a URL to memory.
pub fn get(url: &str) -> Result<Vec<u8>> {
    let fail = |message: String| Error::Fetch {
        url: url.to_owned(),
        message,
    };
    let mut response = ureq::get(url)
        .call()
        .map_err(|error| fail(error.to_string()))?;
    response
        .body_mut()
        .with_config()
        .limit(256 * 1024 * 1024)
        .read_to_vec()
        .map_err(|error| fail(error.to_string()))
}

/// Downloads and parses the newest release's index.
pub fn latest_index(owner_repo: &str) -> Result<Index> {
    let url = index_url(owner_repo);
    let bytes = get(&url)?;
    let text = String::from_utf8(bytes).map_err(|error| Error::Fetch {
        url,
        message: error.to_string(),
    })?;
    Index::parse(&text)
}

/// The newest index when it is newer than the one held, else `None`.
pub fn update_for(held: &Index, owner_repo: &str) -> Result<Option<Index>> {
    let latest = latest_index(owner_repo)?;
    Ok((latest.catalogue.version > held.catalogue.version).then_some(latest))
}

/// Downloads one `.syx` (or any file) pinned to the commit an index was built
/// from.
pub fn file(index: &Index, owner_repo: &str, file: &str) -> Result<Vec<u8>> {
    get(&index.raw_url(owner_repo, file))
}

/// Downloads a `.tar.gz` asset and unpacks it into a folder.
///
/// `patches.tar.gz` unpacks to `<dest>/presets/...`, `demos.tar.gz` to
/// `<dest>/demos/...`.
pub fn unpack(url: &str, dest: &Path) -> Result<()> {
    let bytes = get(url)?;
    let decoder = flate2::read::GzDecoder::new(bytes.as_slice());
    let mut archive = tar::Archive::new(decoder);
    archive.unpack(dest).map_err(|error| Error::io(dest, error))
}

/// Reads a `.tar.gz` already in memory, yielding `(path, bytes)` per file.
pub fn entries(archive: &[u8]) -> Result<Vec<(String, Vec<u8>)>> {
    let decoder = flate2::read::GzDecoder::new(archive);
    let mut archive = tar::Archive::new(decoder);
    let mut out = Vec::new();
    let entries = archive
        .entries()
        .map_err(|error| Error::io("archive", error))?;
    for entry in entries {
        let mut entry = entry.map_err(|error| Error::io("archive", error))?;
        if !entry.header().entry_type().is_file() {
            continue;
        }
        let path = entry
            .path()
            .map_err(|error| Error::io("archive", error))?
            .to_string_lossy()
            .into_owned();
        let mut bytes = Vec::new();
        entry
            .read_to_end(&mut bytes)
            .map_err(|error| Error::io(path.clone(), error))?;
        out.push((path, bytes));
    }
    Ok(out)
}
