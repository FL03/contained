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
pub enum MusicError {
    Custom(String),
    IntervalError(String),
    IOError(String),
    #[default]
    PitchError,
    StdError(String),
}

impl std::error::Error for MusicError {}

impl From<serde_json::Error> for MusicError {
    fn from(error: serde_json::Error) -> Self {
        MusicError::IOError(error.to_string())
    }
}

impl From<std::io::Error> for MusicError {
    fn from(error: std::io::Error) -> Self {
        MusicError::IOError(error.to_string())
    }
}

impl From<Box<dyn std::error::Error>> for MusicError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        MusicError::StdError(error.to_string())
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for MusicError {
    fn from(error: Box<dyn std::error::Error + Send + Sync>) -> Self {
        MusicError::StdError(error.to_string())
    }
}
