/*
    Appellation: errors <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumIter, EnumVariantNames};

#[derive(
    Clone,
    Debug,
    Deserialize,
    Display,
    EnumIter,
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
    CompositionError(String),
    IntervalError(String),
    IOError(String),
    #[default]
    PitchError(String),
    StdError(String),
    TransformationError(String),
}

impl std::error::Error for MusicError {}

impl From<&str> for MusicError {
    fn from(error: &str) -> Self {
        MusicError::StdError(error.to_string())
    }
}

impl From<String> for MusicError {
    fn from(error: String) -> Self {
        MusicError::StdError(error)
    }
}

impl From<anyhow::Error> for MusicError {
    fn from(error: anyhow::Error) -> Self {
        MusicError::StdError(error.to_string())
    }
}

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

impl<E> From<Box<E>> for MusicError
where
    E: std::error::Error,
{
    fn from(error: Box<E>) -> Self {
        MusicError::StdError(error.to_string())
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
