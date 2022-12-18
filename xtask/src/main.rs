/*
    Appellation: xtask <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use clap::Parser;
use scsys_xtask::cmd;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("Welcome to xtask...");

    let handle = std::thread::spawn(move || {
        scsys_xtask::cli::handle().join().unwrap();
    });
    handle.join().ok().unwrap();

    Ok(())
}
