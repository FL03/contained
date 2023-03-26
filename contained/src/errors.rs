/*
    Appellation: errors <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumString, EnumVariantNames};



#[derive(Clone, Debug, Deserialize, Display, EnumString, EnumVariantNames, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, SmartDefault)]
pub enum Error {
    ConncectionError(String),
    #[default]
    Error(String),
    IOError(String),
    Incomplete,
    InvalidLength,
    InvalidType,
    InvalidData,
    SerdeError(String),
}

impl std::error::Error for Error {}

impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        Self::Error(err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::SerdeError(err.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IOError(err.to_string())
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for Error {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Self::Error(err.to_string())
    }
}
