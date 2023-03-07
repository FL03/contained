/*
    Appellation: contained <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use contained::net::{node::Node, NetResult};

#[tokio::main]
async fn main() -> NetResult {
    let node = Node::new();
    node.start(Default::default()).await?;

    Ok(())
}
