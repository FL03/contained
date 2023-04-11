/*
    Appellation: neo <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use contained_sdk as contained;

use contained::music::neo::triads::{Surface, Triad, TriadClass};

#[tokio::main]
async fn main() -> scsys::prelude::AsyncResult {
    let triad = Triad::new(0.into(), TriadClass::Major);
    let _surface = Surface::new(triad);
    Ok(())
}
