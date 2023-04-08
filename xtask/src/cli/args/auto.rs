/*
    Appellation: auto <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::Build;
use crate::command;
use anyhow::Result;
use clap::Args;
use serde::{Deserialize, Serialize};



#[derive(
    Args, Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct Auto;

impl Auto {
    pub fn cargo() -> Result<()> {
        command("cargo", vec!["fmt", "--all"])?;
        command("cargo", vec!["clippy", "--all", "--allow-dirty", "--fix"])?;
        command("cargo", vec!["build", "--workspace"])?;
        command("cargo", vec!["test", "--all"])?;
        Ok(())
    }
}

