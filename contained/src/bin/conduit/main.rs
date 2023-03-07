/*
    Appellation: contained <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use contained::net::{backend::Backend, NetResult};

#[tokio::main]
async fn main() -> NetResult {
    let backend = Backend::new();
    backend.start(Default::default()).await?;

    Ok(())
}
