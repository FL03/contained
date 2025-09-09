/*
    appellation: error <module>
    authors: @FL03
*/
//! this module defines the core error type for the crate

/// a type alias for a [`Result`](core::result::Result) configured to use the custom [`Error`] type.
pub type Result<T> = core::result::Result<T, Error>;

/// The custom error type for the crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[cfg(feature = "alloc")]
    #[error(transparent)]
    BoxError(#[from] alloc::boxed::Box<dyn core::error::Error + Send + Sync + 'static>),
    #[error(transparent)]
    FmtError(#[from] core::fmt::Error),
    #[cfg(feature = "std")]
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[cfg(feature = "alloc")]
    #[error("Unknown Error: {0}")]
    Unknown(alloc::string::String),
}

#[cfg(feature = "alloc")]
mod impl_alloc {
    use super::Error;
    use alloc::boxed::Box;
    use alloc::string::String;

    impl Error {
        pub fn box_error<E>(error: E) -> Self
        where
            E: core::error::Error + Send + Sync + 'static,
        {
            Self::BoxError(Box::new(error))
        }
    }

    impl From<&str> for Error {
        fn from(value: &str) -> Self {
            Self::Unknown(String::from(value))
        }
    }

    impl From<String> for Error {
        fn from(value: String) -> Self {
            Self::Unknown(value)
        }
    }
}
