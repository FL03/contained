/*
    Appellation: xtask <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use xtask_sdk::Xtask;

fn main() -> anyhow::Result<()> {
    let xtask = Xtask::new(Default::default());
    xtask.init();
    xtask.handle_cli(Default::default())?;

    Ok(())
}
