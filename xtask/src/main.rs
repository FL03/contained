/*
    Appellation: scsys-xtask <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use clap::Parser;
use scsys_xtask::cmd;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("Welcome to xtask...");
    let handle = std::thread::spawn(move || {
        cicd().join().ok().unwrap();
    });
    handle.join().ok().unwrap();

    Ok(())
}

fn build_stage() -> std::thread::JoinHandle<()> {
    tracing::info!("Building the workspace...");
    std::thread::spawn(move || {
        cmd!("cargo"; ["build", "--workspace"]);
        cmd!("cargo"; ["test", "--all", "--all-features", "--release"]);
    })
}

fn testing() -> std::thread::JoinHandle<()> {
    tracing::info!("Testing the workspace...");
    std::thread::spawn(move || {
        cmd!("cargo"; ["test", "--all", "--all-features", "--release"]);
    })
}

fn cicd() -> std::thread::JoinHandle<()> {
    tracing::info!("Initializing the CI/CD pipeline");
    std::thread::spawn(move || {
        tracing::info!("Formatting the codespace...");
        cmd!("cargo"; ["fmt", "--all"]);
        tracing::info!("Analyzing the codespace...");
        cmd!("cargo"; ["clippy", "--all", "--allow-dirty", "--fix"]);
        build_stage().join().ok().unwrap();
        testing().join().ok().unwrap();
    })
}

#[derive(Parser)]
pub struct CLI {}
