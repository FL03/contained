/*
    Appellation: reqres <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{codec::*, request::*, response::*};

mod codec;
mod request;
mod response;

use libp2p::request_response::ProtocolName;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumString, EnumVariantNames};

#[derive(
    Clone,
    Copy,
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
#[strum(serialize_all = "snake_case")]
pub enum Proto {
    #[default]
    Cluster,
}

impl ProtocolName for Proto {
    fn protocol_name(&self) -> &[u8] {
        match self {
            Proto::Cluster => b"/cluster/1.0.0",
        }
    }
}
