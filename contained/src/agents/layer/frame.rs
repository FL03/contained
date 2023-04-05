/*
    Appellation: frame <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: The frame describes each unit of data that is processed by the agents
*/
use crate::music::neo::LPR;
use crate::prelude::Error;
use bytes::{Buf, Bytes};
use decanter::prelude::{Hashable, H256};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, EnumVariantNames};

#[derive(
    Clone,
    Debug,
    Deserialize,
    Display,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    Hashable,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum Frame {
    Content {
        cid: H256,
        content: Vec<Bytes>
    },
    Dirac {
        dirac: LPR,
    },
    FuncCall {
        module: H256,
        function: String,
        args: Vec<String>,
    },
    WasmBytes {
        bytes: Bytes,
    },
    Error(Error),
}

impl Frame {
    pub fn content(cid: H256, content: Vec<Bytes>) -> Self {
        Self::Content{ cid, content }
    }
    pub fn dirac(dirac: LPR) -> Self {
        Self::Dirac{ dirac }
    }
    pub fn func_call(module: H256, function: String, args: Vec<String>) -> Self {
        Self::FuncCall{ module, function, args }
    }
    pub fn wasm_bytes(bytes: Bytes) -> Self {
        Self::WasmBytes {
            bytes
        }
    }
    pub fn check(buf: &mut impl Buf) -> Result<(), Error> {
        // Check if the buffer has enough data to read the length
        if buf.remaining() < 4 {
            return Err(Error::Incomplete("Buffer is too small".into()));
        }

        // Read the length
        let len = buf.get_u32();

        // Check if the buffer has enough data to read the frame
        if buf.remaining() < len as usize {
            return Err(Error::Incomplete("Buffer is too small".into()));
        }

        Ok(())
    }
    pub fn parse(buf: &mut impl Buf) -> Result<Self, Error> {
        // Check if the buffer has enough data to read the length
        if buf.remaining() < 4 {
            return Err(Error::Incomplete("Buffer is too small".into()));
        }

        // Read the length
        let len = buf.get_u32();

        // Check if the buffer has enough data to read the frame
        if buf.remaining() < len as usize {
            return Err(Error::Incomplete("Buffer is too small".into()));
        }

        // Read the frame type
        let frame_type = buf.get_u8();

        // Read the frame data
        let data = buf.copy_to_bytes(len as usize - 1);

        // Parse the frame
        let frame = match frame_type {
            0 => {
                let (cid, content) = serde_json::from_slice::<(H256, Vec<Bytes>)>(&data)?;
                Self::content(cid, content)
            }
            1 => {
                let dirac = serde_json::from_slice::<LPR>(&data)?;
                Self::dirac(dirac)
            }
            2 => {
                let (module, function, args) = serde_json::from_slice::<(H256, String, Vec<String>)>(&data)?;
                Self::func_call(module, function, args)
            }
            3 => {
                let wasm_bytes = Bytes::from(data);
                Self::wasm_bytes(wasm_bytes)
            }
            _ => {
                let error = serde_json::from_slice::<Error>(&data)?;
                Self::Error(error)
            }
        };
        Ok(frame)
    }
}
