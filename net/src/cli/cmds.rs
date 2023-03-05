/*
    Appellation: cmds <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::NetResult;
use clap::Subcommand;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use std::path::PathBuf;
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
pub enum Command {
    Provide {
        #[clap(long, short)]
        name: String,
        #[clap(long, short)]
        path: PathBuf,
    },
    #[default]
    Get {
        #[clap(long, short)]
        name: String,
    },
}

impl Command {
    pub fn handle(&self) -> NetResult {
        Ok(())
    }
}
