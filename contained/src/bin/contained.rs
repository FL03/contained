/*
    Appellation: contained <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use contained_sdk::prelude::*;

use tokio::runtime;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let _ = Backend::new()
        .setup()
        .spawn(&runtime::Handle::current())
        .await;
    Ok(())
}
