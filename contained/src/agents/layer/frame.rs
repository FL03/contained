/*
    Appellation: frame <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: The frame describes each unit of data that is processed by the agents
*/
use crate::music::neo::LPR;
use bytes::Bytes;
use decanter::prelude::Hashable;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, EnumVariantNames};

#[derive(Clone, Debug, Deserialize, Display, EnumString, EnumVariantNames, Eq, Hash, Hashable, Ord, PartialEq, PartialOrd, Serialize)]
#[strum(serialize_all = "snake_case")]
pub enum AgentFrame {
    Dirac(LPR),
    WasmBytes(Bytes),
}