/*
    Appellation: contained <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use contained::net::{nodes::Node, NetResult};

use contained::net::peers::Peer;

#[tokio::main]
async fn main() -> NetResult {
    let peer = Peer::default();
    let node = Node::from(peer);
    node.start(Default::default()).await?;

    Ok(())
}
