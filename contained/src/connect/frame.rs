/*
    Appellation: frame <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A frame is used to describe units of data shared between two peers. Implementing a custom framing layer is useful for managing the various types of data that can be sent between peers. 
        This module provides a `Frame` enum that can be used to describe the various types of data that can be sent between peers. The `Frame` enum is used to implement a custom framing layer for 
        the `Connection` type.
*/
use crate::core::Error;
use crate::WorkloadId;
use bytes::Buf;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Frame {
    Empty,
    Triad { id: u32, value: i32 },
    Workload { id: WorkloadId, module: u32 },
}

impl Frame {
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
            0 => Ok(Self::Empty),
            1 => {
                // Parse the triad
                let (id, value) = serde_json::from_slice::<(u32, i32)>(&data)?;

                Ok(Self::Triad { id, value })
            }
            2 => {
                // Parse the workload
                let (id, module) = serde_json::from_slice::<(WorkloadId, u32)>(&data)?;

                Ok(Self::Workload { id, module })
            }
            _ => Err(Error::InvalidType),
        }
    }
}
