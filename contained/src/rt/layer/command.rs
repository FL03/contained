/*
    Appellation: command <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
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
#[strum(serialize_all = "title_case")]
pub enum Command {
    AddTriad {
        id: SpaceId,
        value: u32,
    },
    RemoveTriad {
        id: SpaceId,
    },
    AddWorkload {
        id: WorkloadId,
        module: u32,
    },
    RemoveWorkload {
        id: WorkloadId,
    },
    RunWorkload {
        triad_id: SpaceId,
        workload_id: WorkloadId,
    },
    #[default]
    None,
}

impl Command {
    pub fn add_triad(id: SpaceId, value: u32) -> Self {
        Self::AddTriad { id, value }
    }
    pub fn remove_triad(id: SpaceId) -> Self {
        Self::RemoveTriad { id }
    }
    pub fn add_workload(id: WorkloadId, module: u32) -> Self {
        Self::AddWorkload { id, module }
    }
    pub fn remove_workload(id: WorkloadId) -> Self {
        Self::RemoveWorkload { id }
    }
    pub fn run_workload(triad_id: SpaceId, workload_id: WorkloadId) -> Self {
        Self::RunWorkload {
            triad_id,
            workload_id,
        }
    }
}
