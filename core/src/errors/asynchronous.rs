/*
    Appellation: error <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::Error;
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
pub enum AsyncError {
    BufError(String),
    CapacityError(String),
    ConnectionError(String),
    #[default]
    Error(String),
    IOError(String),
    RecvError(String),
    RuntimeError(String),
    SendError(String),
    SyncError(String),
}

impl AsyncError {
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            AsyncError::Error(e) => e.as_bytes(),
            AsyncError::IOError(e) => e.as_bytes(),
            AsyncError::BufError(e) => e.as_bytes(),
            AsyncError::ConnectionError(e) => e.as_bytes(),
            AsyncError::RecvError(e) => e.as_bytes(),
            AsyncError::SendError(e) => e.as_bytes(),
            AsyncError::RuntimeError(e) => e.as_bytes(),
            AsyncError::SyncError(e) => e.as_bytes(),
            AsyncError::CapacityError(e) => e.as_bytes(),
        }
    }
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

impl std::error::Error for AsyncError {}

impl From<AsyncError> for Error {
    fn from(error: AsyncError) -> Self {
        Self::AsyncError(error)
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for AsyncError {
    fn from(error: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Self::Error(error.to_string())
    }
}

impl From<anyhow::Error> for AsyncError {
    fn from(error: anyhow::Error) -> Self {
        Self::Error(error.to_string())
    }
}

impl From<serde_json::Error> for AsyncError {
    fn from(error: serde_json::Error) -> Self {
        Self::Error(error.to_string())
    }
}

impl From<tokio::io::Error> for AsyncError {
    fn from(error: tokio::io::Error) -> Self {
        Self::IOError(error.to_string())
    }
}

impl From<tokio::net::tcp::ReuniteError> for AsyncError {
    fn from(error: tokio::net::tcp::ReuniteError) -> Self {
        Self::ConnectionError(error.to_string())
    }
}

impl From<tokio::sync::AcquireError> for AsyncError {
    fn from(error: tokio::sync::AcquireError) -> Self {
        Self::ConnectionError(error.to_string())
    }
}

impl<T> From<tokio::sync::SetError<T>> for AsyncError {
    fn from(error: tokio::sync::SetError<T>) -> Self {
        Self::ConnectionError(error.to_string())
    }
}

impl<T> From<tokio::sync::mpsc::error::SendError<T>> for AsyncError {
    fn from(error: tokio::sync::mpsc::error::SendError<T>) -> Self {
        Self::SendError(error.to_string())
    }
}

impl From<tokio::sync::oneshot::error::RecvError> for AsyncError {
    fn from(error: tokio::sync::oneshot::error::RecvError) -> Self {
        Self::RecvError(error.to_string())
    }
}
