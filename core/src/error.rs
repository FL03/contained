/*
    appellation: error <module>
    authors: @FL03
*/
//! this module defines the core error type for the crate

#[cfg(feature = "alloc")]
use alloc::{boxed::Box, string::String};
/// a type alias for a [`Result`](core::result::Result) configured to use the custom [`Error`] type.
pub type Result<T> = core::result::Result<T, Error>;

/// The custom error type for the crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[cfg(feature = "alloc")]
    #[error(transparent)]
    BoxError(#[from] Box<dyn core::error::Error + Send + Sync + 'static>),
    #[error(transparent)]
    FmtError(#[from] core::fmt::Error),
    #[cfg(feature = "std")]
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[cfg(feature = "alloc")]
    #[error("Unknown Error: {0}")]
    Unknown(String),
}

#[cfg(feature = "alloc")]
impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::Unknown(String::from(value))
    }
}

#[cfg(feature = "alloc")]
impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::Unknown(value)
    }
}
