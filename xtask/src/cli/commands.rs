/*
    Appellation: commands <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{command, copy_dir_all, dist_dir, execute_bundle, project_root, Bundle};
use clap::Subcommand;
use std::{sync::Arc, thread::JoinHandle};

#[derive(Clone, Debug, Hash, PartialEq, Subcommand)]
pub enum Commands {
    Auto,
    Compile {
        #[arg(action = clap::ArgAction::SetTrue, long, short)]
        workspace: bool,
    },
    Run {
        #[clap(long, short, value_parser)]
        package: Option<String>,
    },
    Setup {
        #[arg(action = clap::ArgAction::SetTrue, long, short)]
        extras: bool,
    },
}

impl Commands {
    pub fn handler(&self, desktop: bool, release: bool) -> anyhow::Result<&Self> {
        tracing::info!("Processing commands issued to the cli...");
        match self {
            Self::Auto => {
                cicd();
            }
            Self::Compile { workspace } => {
                tracing::info!("Compiling the codebase...");
                setup_artifacts()?;
                build_stage();
            }
            Self::Run { package } => {
                runner(release.clone());
            }
            Self::Setup { extras } => {
                tracing::info!("Setting up the environment...");
                setup_langspace(*extras)?;
            }
        };
        Ok(self)
    }
}

fn setup_artifacts() -> anyhow::Result<()> {
    if std::fs::create_dir_all(&dist_dir()).is_err() {
        tracing::info!("Clearing out the previous build");
        std::fs::remove_dir_all(&dist_dir())?;
        std::fs::create_dir_all(&dist_dir())?;
    };
    Ok(())
}

fn setup_langspace(extras: bool) -> anyhow::Result<()> {
    command("rustup", vec!["default", "nightly"])?;
    command(
        "rustup",
        vec![
            "target",
            "add",
            "wasm32-unknown-unknown",
            "wasm32-wasi",
            "--toolchain",
            "nightly",
        ],
    )?;
    if extras {
        command(
            "rustup",
            vec![
                "component",
                "add",
                "clippy",
                "rustfmt",
                "--toolchain",
                "nightly",
            ],
        )?;
        command("npm", vec!["install", "-g", "wasm-pack"])?;
    };
    Ok(())
}

fn build_stage() {
    tracing::info!("Building the workspace...");

    command("cargo", vec!["build", "--workspace"]);
    command(
        "cargo",
        vec!["test", "--all", "--all-features", "--release"],
    );
}

fn testing() {
    tracing::info!("Testing the workspace...");

    command(
        "cargo",
        vec!["test", "--all", "--all-features", "--release"],
    );
}

fn runner(release: bool) {
    tracing::info!("Initializing the application...");
    let mut args = vec!["run"];
    if release {
        args.push("--release");
    }
    args.push("--");
    args.push("--h");
    command("cargo", args.clone());
}

fn cicd() {
    tracing::info!("Initializing the CI/CD pipeline");
    tracing::info!("Formatting the codespace...");
    command("cargo", vec!["fmt", "--all"]);
    tracing::info!("Analyzing the codespace...");
    command("cargo", vec!["clippy", "--all", "--allow-dirty", "--fix"]);
    build_stage();
    testing();
}
