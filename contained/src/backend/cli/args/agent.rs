/*
    Appellation: agent <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::music::neo::LPR;
use clap::{ArgAction, Args, Subcommand};
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use std::path::PathBuf;
use strum::{Display, EnumVariantNames};

#[derive(
    Args, Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct AgentArgs {
    #[clap(subcommand)]
    pub cmd: Option<AgentOpts>,
    #[arg(action = ArgAction::SetTrue, long, short)]
    pub detached: bool,
    #[clap(long, short)]
    pub port: Option<u16>,
    #[arg(action = ArgAction::SetTrue, long, short)]
    pub up: bool,
}

#[derive(
    Clone,
    Debug,
    Deserialize,
    Display,
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
pub enum AgentOpts {
    Include {
        #[clap(long, short)]
        bytes: Option<Vec<u8>>,
        #[clap(long, short)]
        file: Option<PathBuf>,
    },
    #[default]
    Ping,
    Transform {
        #[clap(long, short)]
        dirac: LPR,
    },
}
