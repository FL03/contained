/*
    Appellation: errors <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
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
    AddrError(String),
    DecodeError(String),
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

impl From<Box<dyn std::error::Error + Send + Sync>> for NetworkError {
    fn from(error: Box<dyn std::error::Error + Send + Sync>) -> Self {
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

impl From<libp2p::core::transport::TransportError<Self>> for NetworkError {
    fn from(error: libp2p::core::transport::TransportError<Self>) -> Self {
        Self::TransportError(error.to_string())
    }
}

impl From<libp2p::core::upgrade::UpgradeError<Self>> for NetworkError {
    fn from(error: libp2p::core::upgrade::UpgradeError<Self>) -> Self {
        Self::UpgradeError(error.to_string())
    }
}
