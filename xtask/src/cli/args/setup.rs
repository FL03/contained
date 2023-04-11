/*
    Appellation: setup <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use anyhow::Result;
use crate::{command, dist_dir, rustup,};
use clap::{Args, ArgAction, ValueEnum};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, EnumVariantNames};

#[derive(
    Clone, Debug, Default, Deserialize, Display, EnumString, EnumVariantNames, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, ValueEnum
)]
#[strum(serialize_all = "kebab-case")]
pub enum PlatformOpts {
    #[default]
    Linux,
    MacOSX,
    Windows,
}

#[derive(
    Args, Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct Setup {
    /// Quickly setup a development environment
    #[arg(action = ArgAction::SetTrue, long, short)]
    pub extras: bool,
    /// Choose a platform to setup
    #[clap(long, short)]
    pub platform: PlatformOpts,
    /// Setup the workspace for WebAssembly workflows
    #[arg(action = ArgAction::SetTrue, long, short)]
    pub wasm: bool
}

impl Setup {
    pub fn new(platform: PlatformOpts) -> Self {
        Self {
            extras: false,
            platform,
            wasm: false
        }
    }
    pub fn clear(&self) -> Result<()> {
        if std::fs::create_dir_all(dist_dir()).is_err() {
            tracing::info!("Clearing out the previous build");
            std::fs::remove_dir_all(dist_dir())?;
            std::fs::create_dir_all(dist_dir())?;
        };
        Ok(())
    }
    pub fn setup(&self) -> Result<()> {
        self.clear()?;
    
        rustup(vec!["install", "nightly"])?;
    
        if self.wasm {
            rustup(vec!["default", "nightly"])?;
            rustup(vec![
                "target",
                "add",
                "wasm32-unknown-unknown",
                "wasm32-wasi",
                "--toolchain",
                "nightly",
            ])?;
            command("npm", vec!["install", "-g", "wasm-pack"])?;
            if self.extras {
                rustup(vec![
                    "component",
                    "add",
                    "clippy",
                    "rustfmt",
                    "--toolchain",
                    "nightly",
                ])?;
            };
        }
    
        Ok(())
    }
    pub fn toggle_extras(mut self, extras: bool) -> Self {
        self.extras = extras;
        self
    }
    pub fn toggle_wasm(mut self, wasm: bool) -> Self {
        self.wasm = wasm;
        self
    }
}