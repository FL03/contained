/*
    Appellation: command <agents>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: This module implements the command for the agents
*/
use super::Responder;
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
        sender: Responder<BoxedWasmValue>,
    },
    Include {
        bytes: Vec<u8>,
        sender: Responder<H256>,
    },
    Transform {
        id: H256,
        dirac: LPR,
        sender: Responder,
    },
}

impl Command {
    pub fn execute(
        module: H256,
        function: String,
        args: BoxedWasmValue,
        with: Option<Imports>,
        sender: Responder<BoxedWasmValue>,
    ) -> Self {
        Self::Execute {
            module,
            function,
            args,
            with,
            sender,
        }
    }
    pub fn include(bytes: Vec<u8>, sender: Responder<H256>) -> Self {
        Self::Include { bytes, sender }
    }
    pub fn transform(id: H256, dirac: LPR, sender: Responder) -> Self {
        Self::Transform { id, dirac, sender }
    }
}
