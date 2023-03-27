/*
    Appellation: response <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
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
    TriadAdded(u32),
    TriadRemoved(u32),
    WorkloadAdded(u32),
    WorkloadRemoved(u32),
    WorkloadRun(u32, u32),
    #[default]
    None,
}
