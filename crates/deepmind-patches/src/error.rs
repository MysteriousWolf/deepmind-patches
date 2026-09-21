//! The crate-wide error type.

use std::path::PathBuf;

/// Anything this crate can fail with.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// Reading or writing a file failed.
    #[error("{path}: {source}")]
    Io {
        /// The file.
        path: PathBuf,
        /// The underlying error.
        #[source]
        source: std::io::Error,
    },
    /// A TOML file did not parse.
    #[error("{path}: {message}")]
    Toml {
        /// The file.
        path: PathBuf,
        /// The parser's message, which names the line.
        message: String,
    },
    /// A `.syx` file did not hold exactly one program.
    #[error("{path}: expected one program dump, found {found}")]
    ProgramCount {
        /// The file.
        path: PathBuf,
        /// How many it held.
        found: usize,
    },
    /// The underlying protocol library refused something.
    #[error(transparent)]
    Midi(#[from] deepmind_midi::Error),
    /// An icon row was not seven of `#` or `.`.
    #[error("icon: {0}")]
    Icon(String),
    /// A version string was not `yy.release.patch`.
    #[error("version: {0}")]
    Version(String),
    /// A fingerprint string was not 64 hex digits.
    #[error("fingerprint: {0}")]
    Fingerprint(String),
    /// An index carries a schema this crate does not read.
    #[error("index schema {found} is newer than the {supported} this crate reads")]
    Schema {
        /// What the index says.
        found: u32,
        /// What this crate reads.
        supported: u32,
    },
    /// An MP3 file could not be read as one.
    #[error("{path}: {message}")]
    Mp3 {
        /// The file.
        path: PathBuf,
        /// What was wrong.
        message: String,
    },
    /// Validation found errors. The findings carry the detail.
    #[error("{0} validation error(s)")]
    Invalid(usize),
    /// A network operation failed (feature `fetch`).
    #[error("fetch {url}: {message}")]
    Fetch {
        /// What was requested.
        url: String,
        /// What went wrong.
        message: String,
    },
}

/// `Result` with this crate's [`Error`].
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub(crate) fn io(path: impl Into<PathBuf>, source: std::io::Error) -> Self {
        Self::Io {
            path: path.into(),
            source,
        }
    }
}
