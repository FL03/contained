/*
    Appellation: event <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::{EnvId, WorkloadId};
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
pub enum ClusterEvent {
    TriadAdded {
        id: EnvId,
    },
    TriadRemoved {
        id: EnvId,
    },
    WorkloadAdded {
        id: WorkloadId,
    },
    WorkloadRemoved {
        id: WorkloadId,
    },
    WorkloadRun {
        triad_id: EnvId,
        workload_id: WorkloadId,
    },
    #[default]
    None,
}

impl ClusterEvent {
    pub fn triad_added(id: EnvId) -> Self {
        Self::TriadAdded { id }
    }
    pub fn triad_removed(id: EnvId) -> Self {
        Self::TriadRemoved { id }
    }
    pub fn workload_added(id: WorkloadId) -> Self {
        Self::WorkloadAdded { id }
    }
    pub fn workload_removed(id: WorkloadId) -> Self {
        Self::WorkloadRemoved { id }
    }
    pub fn workload_run(triad_id: EnvId, workload_id: WorkloadId) -> Self {
        Self::WorkloadRun {
            triad_id,
            workload_id,
        }
    }
}
