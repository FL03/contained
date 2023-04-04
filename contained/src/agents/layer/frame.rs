/*
    Appellation: frame <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: The frame describes each unit of data that is processed by the agents
*/
use crate::music::neo::LPR;
use crate::prelude::Error;
use bytes::{Buf, Bytes};
use decanter::prelude::Hashable;
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
    Dirac(LPR),
    WasmBytes(Bytes),
    Error(Error),
}

impl Frame {
    pub fn dirac(dirac: LPR) -> Self {
        Self::Dirac(dirac)
    }
    pub fn wasm_bytes(bytes: Bytes) -> Self {
        Self::WasmBytes(bytes)
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
        match frame_type {
            0 => {
                // Parse the dirac
                let dirac = serde_json::from_slice::<LPR>(&data)?;

                Ok(Self::Dirac(dirac))
            }
            1 => {
                // Parse the wasm bytes
                let wasm_bytes = Bytes::from(data);

                Ok(Self::WasmBytes(wasm_bytes))
            }
            _ => {
                // Parse the error
                let error = serde_json::from_slice::<Error>(&data)?;

                Ok(Self::Error(error))
            }
        }
    }
}
