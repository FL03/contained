/*
    Appellation: opts <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use clap::Subcommand;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumString, EnumVariantNames};

#[derive(
    Clone,
    Debug,
    Display,
    Deserialize,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    SmartDefault,
    Subcommand,
)]
#[strum(serialize_all = "snake_case")]
pub enum Opts {
    #[default]
    Run {
        #[clap(long, short)]
        release: bool,
    },
}
