/*
    Appellation: response <module>
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
pub enum Response {
    TriadAdded {
        id: u32
    },
    TriadRemoved {
        id: u32
    },
    WorkloadAdded {
        id: WorkloadId
    },
    WorkloadRemoved {
        id: WorkloadId
    },
    WorkloadRun {
        triad_id: u32,
        workload_id: WorkloadId,
    },
    #[default]
    None,
}
