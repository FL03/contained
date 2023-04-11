/*
    Appellation: reqres <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{codec::*, request::*, response::*};

mod codec;
mod request;
mod response;

use crate::NetworkError;
use libp2p::request_response::{self, ProtocolName, ProtocolSupport};
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumString, EnumVariantNames};

pub use libp2p::request_response::RequestId;

pub type ProtoBehaviour = request_response::Behaviour<ProtocolCodec>;

pub type ReqResEvent = request_response::Event<Request, Response>;

pub fn new() -> ProtoBehaviour {
    request_response::Behaviour::new(
        ProtocolCodec::default(),
        std::iter::once((Proto::Cluster, ProtocolSupport::Full)),
        Default::default(),
    )
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
#[strum(serialize_all = "snake_case")]
pub enum Frame {
    Wasm(Vec<u8>),
    #[default]
    Error(NetworkError),
}

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
