#![doc = r"The crate's error type"]

/// The error type returned by all network-backed methods in this crate.
#[derive(Debug, thiserror::Error)]
pub enum OpenLigaError {
    /// A request to the OpenLigaDB API failed, or its response body could not
    /// be deserialized as the expected JSON shape.
    #[error("request to OpenLigaDB failed: {0}")]
    Reqwest(#[from] reqwest::Error),
    /// A URL could not be constructed for the requested endpoint.
    #[error("failed to construct request URL: {0}")]
    Url(#[from] url::ParseError),
}
