/*
    Appellation: neo <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use contained::music::neo::triads::{Triad, Triads};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let _triad = Triad::new(0.into(), Triads::Major);

    Ok(())
}
