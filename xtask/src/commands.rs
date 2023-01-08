/*
    Appellation: commands <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{command, dist_dir, rustup};
use anyhow::Result;

pub fn auto() -> Result<()> {
    command("cargo", vec!["fmt", "--all"])?;
    command("cargo", vec!["clippy", "--all", "--allow-dirty", "--fix"])?;
    for i in [true, false] {
        builder(i, true)?;
    }
    Ok(())
}

pub fn builder(release: bool, workspace: bool) -> Result<()> {
    let mut args = vec!["build"];

    if release {
        args.push("--release");
    }
    if workspace {
        args.push("--workspace");
    }
    command("cargo", args)?;
    Ok(())
}

pub fn setup(extras: bool, wasm: bool) -> Result<()> {
    if std::fs::create_dir_all(dist_dir()).is_err() {
        tracing::info!("Clearing out the previous build");
        std::fs::remove_dir_all(dist_dir())?;
        std::fs::create_dir_all(dist_dir())?;
    };

    rustup(vec!["install", "nightly"])?;

    if wasm {
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
        if extras {
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
