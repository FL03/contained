/*
    Appellation: error <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::AsyncError;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumCount, EnumIs, EnumIter, EnumString, VariantNames};

#[derive(
    Clone,
    Debug,
    Deserialize,
    Display,
    EnumCount,
    EnumIs,
    EnumIter,
    EnumString,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    SmartDefault,
    VariantNames,
)]
#[strum(serialize_all = "title_case")]
pub enum Error {
    AsyncError(AsyncError),
    CapacityError(String),
    CompileError(String),
    ConnectionError(String),
    ExportError(String),
    #[default]
    Error(String),
    ExecutionError(String),
    Incomplete(String),
    RangeError,
    TypeError,
    IOError(String),
    MemoryError(String),
    NotFound,
    RecvError(String),
    SendError(String),
    StateError,
    StoreError,
    TranslateError,
    TransformError,
    TapeError,
    RuntimeError(String),
    ValidationError,
}

impl Error {
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Error::Error(e) => e.as_bytes(),
            Error::IOError(e) => e.as_bytes(),
            Error::AsyncError(e) => e.as_bytes(),
            Error::CompileError(e) => e.as_bytes(),
            Error::ConnectionError(e) => e.as_bytes(),
            Error::ExportError(e) => e.as_bytes(),
            Error::ExecutionError(e) => e.as_bytes(),
            Error::Incomplete(e) => e.as_bytes(),
            Error::MemoryError(e) => e.as_bytes(),
            Error::RecvError(e) => e.as_bytes(),
            Error::SendError(e) => e.as_bytes(),
            Error::RuntimeError(e) => e.as_bytes(),
            Error::TranslateError => b"TranslateError",
            Error::TransformError => b"TransformError",
            Error::TapeError => b"TapeError",
            Error::ValidationError => b"ValidationError",
            Error::RangeError => b"RangeError",
            Error::TypeError => b"TypeError",
            Error::StateError => b"StateError",
            Error::StoreError => b"StoreError",
            Error::CapacityError(e) => e.as_bytes(),
            Error::NotFound => b"NotFound",
        }
    }
}

impl std::error::Error for Error {}

impl From<String> for Error {
    fn from(error: String) -> Self {
        Error::Error(error)
    }
}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        Error::Error(error.to_string())
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for Error {
    fn from(error: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Error::AsyncError(error.into())
    }
}

impl From<anyhow::Error> for Error {
    fn from(error: anyhow::Error) -> Self {
        Error::Error(error.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IOError(error.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::Error(error.to_string())
    }
}

impl<T> From<tokio::sync::mpsc::error::SendError<T>> for Error {
    fn from(error: tokio::sync::mpsc::error::SendError<T>) -> Self {
        Error::AsyncError(error.into())
    }
}

impl From<tokio::sync::oneshot::error::RecvError> for Error {
    fn from(error: tokio::sync::oneshot::error::RecvError) -> Self {
        Error::AsyncError(error.into())
    }
}
