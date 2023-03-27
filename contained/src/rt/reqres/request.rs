/*
    Appellation: request <module>
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
pub enum Request {
    AddTriad(u32, i32),
    RemoveTriad(u32),
    AddWorkload(u32, u32),
    RemoveWorkload(u32),
    RunWorkload(u32, u32),
    #[default]
    None,
}
