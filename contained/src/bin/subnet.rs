/*
    Appellation: subnet <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use contained_sdk as contained;

use contained::net::{NetworkConfig, Overlay, Starter};

#[tokio::main]
async fn main() -> scsys::prelude::AsyncResult {
    let addr = "/ip4/0.0.0.0/tcp/9099".parse().unwrap();
    let cnf = NetworkConfig::new(addr, Some(9));
    let (node, _client, _events) = Starter::new()
        .with_config(cnf)
        .set_overlay(Overlay::Subnet)
        .start();
    node.spawn();
    Ok(())
}
