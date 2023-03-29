/*
    Appellation: event <module>
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
pub enum SystemEvent {
    TriadAdded {
        id: SpaceId,
    },
    TriadRemoved {
        id: SpaceId,
    },
    WorkloadAdded {
        id: WorkloadId,
    },
    WorkloadRemoved {
        id: WorkloadId,
    },
    WorkloadRun {
        triad_id: SpaceId,
        workload_id: WorkloadId,
    },
    #[default]
    None,
}

impl SystemEvent {
    pub fn triad_added(id: SpaceId) -> Self {
        Self::TriadAdded { id }
    }
    pub fn triad_removed(id: SpaceId) -> Self {
        Self::TriadRemoved { id }
    }
    pub fn workload_added(id: WorkloadId) -> Self {
        Self::WorkloadAdded { id }
    }
    pub fn workload_removed(id: WorkloadId) -> Self {
        Self::WorkloadRemoved { id }
    }
    pub fn workload_run(triad_id: SpaceId, workload_id: WorkloadId) -> Self {
        Self::WorkloadRun {
            triad_id,
            workload_id,
        }
    }
}
