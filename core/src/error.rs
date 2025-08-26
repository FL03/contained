/*
    appellation: error <module>
    authors: @FL03
*/
//! this module defines the core error type for the crate

/// a type alias for a [`Result`](core::result::Result) configured to use the custom [`Error`] type.
pub type Result<T> = core::result::Result<T, Error>;

/// The custom error type for the crate.
#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    #[cfg(feature = "alloc")]
    #[error("Unknown Error: {0}")]
    Unknown(String),
}
