/*
    Appellation: command <agents>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: explicit commands for agents
        Commands:
            Execute: execute a function in a module
            Include: include a module
            Transform: transform a module
*/
use super::OneshotSender;
use crate::music::neo::LPR;
use crate::prelude::BoxedWasmValue;
// use decanter::prelude::H256;
use wasmer::Imports;

#[derive(Debug)]
pub enum Command {
    Execute {
        module: String,
        function: String,
        args: BoxedWasmValue,
        with: Option<Imports>,
        tx: OneshotSender<BoxedWasmValue>,
    },
    Include {
        bytes: Vec<u8>,
        tx: OneshotSender<String>,
    },
    Transform {
        id: String,
        dirac: LPR,
        tx: OneshotSender,
    },
}

impl Command {
    pub fn execute(
        module: String,
        function: String,
        args: BoxedWasmValue,
        with: Option<Imports>,
        tx: OneshotSender<BoxedWasmValue>,
    ) -> Self {
        Self::Execute {
            module,
            function,
            args,
            with,
            tx,
        }
    }
    pub fn include(bytes: Vec<u8>, tx: OneshotSender<String>) -> Self {
        Self::Include { bytes, tx }
    }
    pub fn transform(id: String, dirac: LPR, tx: OneshotSender) -> Self {
        Self::Transform { id, dirac, tx }
    }
}
