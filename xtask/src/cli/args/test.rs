/*
    Appellation: build <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::command;
use anyhow::Result;
use clap::Args;
use serde::{Deserialize, Serialize};



#[derive(
    Args, Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct Test {
    #[clap(long, short)]
    pub package: Option<String>,
}

