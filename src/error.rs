use serde_json::Error as JsonError;
use std::io::Error as IOError;

/// An error.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// I/O error.
    #[error("IO Error: {0}")]
    IO(IOError),
    /// JSON error.
    #[error("JSON Error: {0}")]
    Json(JsonError),
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Self {
        Error::Json(err)
    }
}

impl From<IOError> for Error {
    fn from(err: IOError) -> Self {
        Error::IO(err)
    }
}
