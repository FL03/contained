/*
    Appellation: event <agents>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::BoxedWasmValue;
use decanter::prelude::H256;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumString, EnumVariantNames};

#[derive(Debug)]
pub enum CommandEvent {
    Executed { cid: H256, result: BoxedWasmValue },
    Included { cid: H256 },
    Transformed,
}


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
)]
pub enum AgentError {
    ConnectionError(String),
    Error(String),
    ExecutionError(String),
    InitError(String),
    IOError(String),
}

impl std::error::Error for AgentError {}

impl Default for AgentError {
    fn default() -> Self {
        Self::Error("".to_string())
    }
}

impl From<Box<dyn std::error::Error>> for AgentError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        Self::Error(error.to_string())
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for AgentError {
    fn from(error: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Self::Error(error.to_string())
    }
}

impl From<std::io::Error> for AgentError {
    fn from(error: std::io::Error) -> Self {
        Self::IOError(error.to_string())
    }
}

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
)]
pub enum Event {
    Error(AgentError),
}

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
pub enum ConnectionEvent {
    #[default]
    Connecting,
    Disonnecting,
}

pub enum RegistrationEvent {
    Registered,
    Registering,
    Unregistered,
    Unregistering,
}

pub enum Controls {
    Connect,
    Disconnect,
    Register,
    Start,
    Terminate,
}
