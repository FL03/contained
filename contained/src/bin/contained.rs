/*
    Appellation: contained <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use contained_sdk::prelude::*;

use tokio::runtime;

#[tokio::main]
async fn main() -> scsys::prelude::AsyncResult {
    let _ = Backend::new()
        .setup()
        .spawn(&runtime::Handle::current())
        .await;
    Ok(())
}
