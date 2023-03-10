/*
    Appellation: xtask <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use anyhow::Result;
use serde::{Deserialize, Serialize};
use xtask_sdk::{cli::{CommandLineInterface, Opts}, Context, auto, command, setup};

fn main() -> Result<()> {
    let xtask = Xtask::new(Default::default());
    xtask.init();
    xtask.handle_cli(Default::default())?;

    Ok(())
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Xtask {
    ctx: Context
}

impl Xtask {
    pub fn new(ctx: Context) -> Self {
        Self { ctx }
    }
    pub fn handle_cli(&self, cli: CommandLineInterface) -> Result<()> {
        let release = cli.release();
        let workspace = cli.workspace();
        if let Some(opts) = cli.cmd().clone() {
            match opts {
                Opts::Auto => {
                    tracing::info!("Initializing the automatic pipeline");
                    auto()?;
                },
                Opts::Build(_build) => {
                    tracing::info!("Building the target...");
                    let mut args = vec!["build"];

                    if release {
                        args.push("--release");
                    }
                    if workspace {
                        args.push("--workspace");
                    }
                    command("cargo", args)?;
                },
                Opts::Setup(_setup) => {
                    tracing::info!("Setting up the workspace");
                    setup(true, false)?;
                },
                Opts::Test { .. } => {
                    tracing::info!("Testing the target(s)");
                }
            }
        }
        Ok(())
    }
    pub fn init(&self) {
        tracing_subscriber::fmt::init();
    }
    pub async fn run(&self) -> Result<()> {
        self.init();
        self.handle_cli(Default::default())?;
        
        Ok(())
    }
}

