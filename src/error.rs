use thiserror::Error;

/// Crate-wide result alias using [`enum@Error`].
pub type Result<T> = std::result::Result<T, Error>;

/// Crate-wide error type. Add variants as new failure modes appear.
#[derive(Debug, Error)]
pub enum Error {
    /// Configuration could not be loaded or was invalid.
    #[error("configuration error: {0}")]
    Config(String),

    /// The requested resource does not exist.
    #[error("not found: {0}")]
    NotFound(String),
}
