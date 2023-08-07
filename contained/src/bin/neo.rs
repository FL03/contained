/*
    Appellation: neo <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use contained::music::neo::triads::{Triad, Triads};
use contained::prelude::State;
use wasmer::{imports, FunctionEnv, Imports, Store};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let _triad = Triad::new(0.into(), Triads::Major);

    Ok(())
}

pub struct Conduit {
    state: State,
    store: Store,
    triad: Triad,
}
