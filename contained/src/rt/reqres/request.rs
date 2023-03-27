/*
    Appellation: request <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::WorkloadId;
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
pub enum Request {
    AddTriad {
        id: u32,
        value: u32,
    },
    RemoveTriad {
        id: u32,
    },
    AddWorkload {
        id: WorkloadId,
        module: u32,
    },
    RemoveWorkload {
        id: WorkloadId,
    },
    RunWorkload {
        triad_id: u32,
        workload_id: WorkloadId,
    },
    #[default]
    None,
}
