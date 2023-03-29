/*
    Appellation: reqres <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/

pub use self::{command::*, event::*};

mod command;
mod event;

use crate::{SpaceId, WorkloadId};
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
#[repr(i64)]
#[strum(serialize_all = "snake_case")]
pub enum Action {
    Add(Operator),
    Get(Operator),
    #[default]
    Ping(Operator),
    Remove(Operator),
    Run(Operator),
    Update(Operator),
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
pub enum Operator {
    #[default]
    Triad {
        id: SpaceId,
    },
    Workload {
        id: WorkloadId,
    },
}
