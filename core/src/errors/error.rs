/*
    Appellation: error <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::AsyncError;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumString, EnumVariantNames};

#[derive(
    Clone,
    Debug,
    Deserialize,
    Display,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    SmartDefault,
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

impl std::error::Error for Error {}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        Error::Error(error.to_string())
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

impl From<Box<dyn std::error::Error + Send + Sync>> for Error {
    fn from(error: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Error::AsyncError(error.into())
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

impl From<wasmer::CompileError> for Error {
    fn from(error: wasmer::CompileError) -> Self {
        Error::CompileError(error.to_string())
    }
}

impl From<wasmer::ExportError> for Error {
    fn from(error: wasmer::ExportError) -> Self {
        Error::ExportError(error.to_string())
    }
}

impl From<wasmer::RuntimeError> for AsyncError {
    fn from(error: wasmer::RuntimeError) -> Self {
        Self::RuntimeError(error.to_string())
    }
}

impl From<wasmer::WasmError> for Error {
    fn from(error: wasmer::WasmError) -> Self {
        Error::ExecutionError(error.to_string())
    }
}
