/*
    Appellation: setup <args>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{command, dist_dir};
use anyhow::Result;
use clap::Args;
use serde::{Deserialize, Serialize};

#[derive(Args, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Setup {
    #[arg(action = clap::ArgAction::SetTrue, long, short)]
    extras: bool,
    #[arg(action = clap::ArgAction::SetTrue, long, short)]
    wasm: bool,
}

impl Setup {
    pub fn new(extras: bool, wasm: bool) -> Self {
        Self { extras, wasm }
    }
    pub fn setup_artifacts(&self) -> Result<&Self> {
        if std::fs::create_dir_all(&dist_dir()).is_err() {
            tracing::info!("Clearing out the previous build");
            std::fs::remove_dir_all(&dist_dir())?;
            std::fs::create_dir_all(&dist_dir())?;
        };
        Ok(self)
    }
    pub fn langspace(&self) -> Result<&Self> {
        rustup(vec!["install", "nightly"])?;
        
        if self.wasm {
            rustup(vec!["default", "nightly"])?;
            rustup(
                vec![
                    "target",
                    "add",
                    "wasm32-unknown-unknown",
                    "wasm32-wasi",
                    "--toolchain",
                    "nightly",
                ],
            )?;
            command("npm", vec!["install", "-g", "wasm-pack"])?;
            if self.extras {
                rustup(
                    vec![
                        "component",
                        "add",
                        "clippy",
                        "rustfmt",
                        "--toolchain",
                        "nightly",
                    ],
                )?;
            };
        }
        
        Ok(self)
    }
    pub fn handler(&self) -> Result<&Self> {
        tracing::info!("Setting up the workspace...");
        self.setup_artifacts()?.langspace()?;

        Ok(self)
    }
}

fn rustup(args: Vec<&str>) -> Result<()> {
    command("rustup",  args)
}
