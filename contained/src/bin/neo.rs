/*
    Appellation: neo <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use contained_sdk as contained;

use contained::music::neo::triads::{Triad, Triads};

#[tokio::main]
async fn main() -> scsys::prelude::AsyncResult {
    let _triad = Triad::new(0.into(), Triads::Major);

    Ok(())
}
