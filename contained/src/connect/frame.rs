/*
    Appellation: frame <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A frame is used to describe units of data shared between two peers. Implementing a custom framing layer is useful for managing the various types of data that can be sent between peers.
        This module provides a `Frame` enum that can be used to describe the various types of data that can be sent between peers. The `Frame` enum is used to implement a custom framing layer for
        the `Connection` type.
*/
use crate::prelude::Error;
use bytes::{Buf, Bytes};
use serde::{Deserialize, Serialize};
use wasmer::Module;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Frame {
    Bulk(Vec<Bytes>),
    WasmBytes(Bytes),
    Error(Error),
}

impl Frame  {
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
                // Parse the environment
                let data = serde_json::from_slice::<Vec<Bytes>>(&data)?;

                Ok(Self::Bulk(data))
            }
            1 => {
                // Parse the triad
                let workload = serde_json::from_slice::<Bytes>(&data.clone())?;

                Ok(Self::WasmBytes(workload))
            }
            _ => {
                // Parse the error
                let error = serde_json::from_slice::<Error>(&data)?;

                Ok(Self::Error(error))
            }
        }
    }
}
