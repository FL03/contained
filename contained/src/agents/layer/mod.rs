/*
    Appellation: layer <agents>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: This module implements the async layer for the agents
*/
pub use self::frame::*;

mod frame;

use crate::music::neo::LPR;
use crate::prelude::BoxedWasmValue;
use decanter::prelude::H256;
use wasmer::Imports;

#[derive(Debug)]
pub enum Command {
    Execute {
        module: H256,
        function: String,
        args: BoxedWasmValue,
        with: Option<Imports>,
    },
    Include {
        bytes: Vec<u8>,
    },
    Transform {
        id: H256,
        dirac: LPR,
    },
}

impl Command {
    pub fn execute(module: H256, function: String, args: BoxedWasmValue, with: Option<Imports>) -> Self {
        Self::Execute {
            module,
            function,
            args,
            with
        }
    }
    pub fn include(bytes: Vec<u8>) -> Self {
        Self::Include { bytes }
    }
}
