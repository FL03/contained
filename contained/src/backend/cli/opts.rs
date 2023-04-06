/*
    Appellation: opts <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::prelude::PeerId;
use clap::{ArgAction, Args, Subcommand};
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumString, EnumVariantNames};

#[derive(
    Args, Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct Network {
    #[arg(action = ArgAction::SetTrue, long, short)]
    pub detached: bool,
    #[clap(long)]
    pub dial: Option<String>,
    #[clap(long, short)]
    pub listen: Option<String>,
    #[clap(long, short)]
    pub seed: Option<u8>,
    #[arg(action = ArgAction::SetTrue, long, short)]
    pub up: bool,
}

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
    #[strum(disabled)]
    Dial {
        #[clap(long, short)]
        addr: String,
        #[clap(long, short)]
        pid: PeerId,
    },
    Execute {
        #[clap(long, short)]
        space: Option<String>,
        #[clap(long, short)]
        workload: String,
    },
    #[default]
    Network(Network),
    Setup {
        #[clap(long, short)]
        addr: Option<String>,
        #[clap(long, short)]
        host: Option<String>,
        #[clap(long, short)]
        port: Option<u16>,
    },
}
