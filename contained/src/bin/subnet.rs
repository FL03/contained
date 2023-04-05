/*
    Appellation: subnet <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use contained_sdk as contained;

use contained::agents::client::Connect;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> scsys::prelude::AsyncResult {
    // Connect to the contained server.
    let mut connection = Connect::connect("0.0.0.0:8080").await?;
    
    Ok(())
}

