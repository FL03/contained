/*
    Appellation: Contained <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{states::*};

pub(crate) mod states;


use scsys::prelude::AsyncResult;
use std::sync::Arc;

#[tokio::main]
async fn main() -> AsyncResult {
    

    Ok(())
}

pub struct Machine {
    pub states: State,
}


