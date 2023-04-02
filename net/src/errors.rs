/*
    Appellation: errors <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use contained_core::{AsyncError, Error};
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
pub enum NetworkError {
    AsyncError(AsyncError),
    AddrError(String),
    ConnectionError(String),
    DecodeError(String),
    DialError(String),
    EncodeError(String),
    #[default]
    Error(String),
    IOError(String),
    ParseError(String),
    ReqResError(String),
    TransportError(String),
    UpgradeError(String),
}

impl std::error::Error for NetworkError {}

impl From<NetworkError> for Error {
    fn from(error: NetworkError) -> Self {
        Self::AsyncError(error.into())
    }
}

impl From<NetworkError> for AsyncError {
    fn from(error: NetworkError) -> Self {
        Self::Error(error.to_string())
    }
}

impl From<anyhow::Error> for NetworkError {
    fn from(error: anyhow::Error) -> Self {
        Self::Error(error.to_string())
    }
}

impl From<serde_json::Error> for NetworkError {
    fn from(error: serde_json::Error) -> Self {
        Self::Error(error.to_string())
    }
}

impl From<std::io::Error> for NetworkError {
    fn from(error: std::io::Error) -> Self {
        Self::IOError(error.to_string())
    }
}

impl From<Box<dyn std::error::Error>> for NetworkError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        Self::Error(error.to_string())
    }
}

impl From<libp2p::core::DecodeError> for NetworkError {
    fn from(error: libp2p::core::DecodeError) -> Self {
        Self::DecodeError(error.to_string())
    }
}

impl From<libp2p::core::multiaddr::Error> for NetworkError {
    fn from(error: libp2p::core::multiaddr::Error) -> Self {
        Self::AddrError(error.to_string())
    }
}

impl<E> From<libp2p::core::transport::TransportError<E>> for NetworkError
where
    E: std::error::Error,
{
    fn from(error: libp2p::core::transport::TransportError<E>) -> Self {
        Self::TransportError(error.to_string())
    }
}

impl From<libp2p::core::upgrade::UpgradeError<Self>> for NetworkError {
    fn from(error: libp2p::core::upgrade::UpgradeError<Self>) -> Self {
        Self::UpgradeError(error.to_string())
    }
}

impl From<libp2p::swarm::DialError> for NetworkError {
    fn from(error: libp2p::swarm::DialError) -> Self {
        Self::DialError(error.to_string())
    }
}

impl From<libp2p::swarm::ConnectionError<Box<dyn std::error::Error>>> for NetworkError {
    fn from(error: libp2p::swarm::ConnectionError<Box<dyn std::error::Error>>) -> Self {
        Self::ConnectionError(error.to_string())
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for NetworkError {
    fn from(error: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Self::AsyncError(error.into())
    }
}

impl<T> From<tokio::sync::mpsc::error::SendError<T>> for NetworkError {
    fn from(error: tokio::sync::mpsc::error::SendError<T>) -> Self {
        Self::AsyncError(error.into())
    }
}

impl From<tokio::sync::oneshot::error::RecvError> for NetworkError {
    fn from(error: tokio::sync::oneshot::error::RecvError) -> Self {
        Self::AsyncError(error.into())
    }
}
