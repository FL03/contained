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
    Execute {
        #[clap(long, short)]
        space: Option<String>,
        #[clap(long, short)]
        workload: String,
    },
    Setup {
        #[clap(long, short)]
        addr: String,
    },
    #[default]
    Start {
        #[clap(long, short)]
        release: bool,
    },
}
