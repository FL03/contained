/*
    Appellation: contained <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use contained_sdk::prelude::*;

use bytes::{BufMut, BytesMut, Buf};
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumString, EnumVariantNames};
use std::collections::HashMap;
use std::io::Cursor;
use std::sync::{Arc, RwLock};
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::net::TcpStream;
use wasmer::{Instance, Module, Store, imports};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Ok(())
}


// Struct for storing the state of a triadic structure
#[derive(Clone)]
pub struct Surface {
    id: u32,
    value: i32,
}

// Struct for storing the state of a workload
pub struct Workload {
    id: u32,
    module: Module,
}

// Struct for storing the state of the application
pub struct AppState {
    triads: RwLock<HashMap<u32, Instance>>,
    workloads: RwLock<HashMap<u32, Workload>>,
}

#[derive(Clone, Debug, Deserialize,  Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Frame {
    Triad(u32, i32),
    Workload(u32, u32),
}

impl Frame {
    pub fn check(buf: &mut impl Buf) -> Result<(), Error> {
        // Check if the buffer has enough data to read the length
        if buf.remaining() < 4 {
            return Err(Error::Incomplete);
        }

        // Read the length
        let len = buf.get_u32();

        // Check if the buffer has enough data to read the frame
        if buf.remaining() < len as usize {
            return Err(Error::Incomplete);
        }

        Ok(())
    }
    pub fn parse(buf: &mut impl Buf) -> Result<Self, Error> {
        // Check if the buffer has enough data to read the length
        if buf.remaining() < 4 {
            return Err(Error::Incomplete);
        }

        // Read the length
        let len = buf.get_u32();

        // Check if the buffer has enough data to read the frame
        if buf.remaining() < len as usize {
            return Err(Error::Incomplete);
        }

        // Read the frame type
        let frame_type = buf.get_u8();

        // Read the frame data
        let data = buf.copy_to_bytes(len as usize - 1);

        // Parse the frame
        match frame_type {
            0 => {
                // Parse the triad
                let triad = serde_json::from_slice::<(u32, i32)>(&data)?;

                Ok(Self::Triad(triad.0, triad.1))
            }
            1 => {
                // Parse the workload
                let workload = serde_json::from_slice::<(u32, u32)>(&data)?;

                Ok(Self::Workload(workload.0, workload.1))
            }
            _ => Err(Error::InvalidType),
        }
    }
}

pub enum Message {
    Frame(Frame),
    Shutdown,
}

pub enum Operation {
    AddTriad(u32, i32),
    RemoveTriad(u32),
    AddWorkload(u32, u32),
    RemoveWorkload(u32),
    RunWorkload(u32, u32),
}

pub enum Response {
    TriadAdded(u32),
    TriadRemoved(u32),
    WorkloadAdded(u32),
    WorkloadRemoved(u32),
    WorkloadRun(u32, u32),
}

// Struct for handling incoming connections
pub struct ConnectionHandler {
    buffer: BytesMut,
    state: Arc<AppState>,
    stream: TcpStream,
}

impl ConnectionHandler {
    pub fn new(state: Arc<AppState>, stream: TcpStream) -> Self {
        Self {
            // Allocate the buffer with 4kb of capacity.
            buffer: BytesMut::with_capacity(4096),
            state,
            stream,
        }
    }
    fn parse_frame(&mut self) ->  Result<Option<Frame>, Error>
    {
        // Create the `T: Buf` type.
        let mut buf = Cursor::new(&self.buffer[..]);

        // Check whether a full frame is available
        match Frame::check(&mut buf) {
            Ok(_) => {
                // Get the byte length of the frame
                let len = buf.position() as usize;

                // Reset the internal cursor for the
                // call to `parse`.
                buf.set_position(0);

                // Parse the frame
                let frame = Frame::parse(&mut buf)?;

                // Discard the frame from the buffer
                self.buffer.advance(len);

                // Return the frame to the caller.
                Ok(Some(frame))
            }
            // Not enough data has been buffered
            Err(Error::Incomplete) => Ok(None),
            // An error was encountered
            Err(e) => Err(e.into()),
        }
    }
    pub async fn read_frame(&mut self) -> Result<Option<Frame>, Error>
    {
        loop {
            // Attempt to parse a frame from the buffered data. If
            // enough data has been buffered, the frame is
            // returned.
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }

            // There is not enough buffered data to read a frame.
            // Attempt to read more data from the socket.
            //
            // On success, the number of bytes is returned. `0`
            // indicates "end of stream".
            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                // The remote closed the connection. For this to be
                // a clean shutdown, there should be no data in the
                // read buffer. If there is, this means that the
                // peer closed the socket while sending a frame.
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err(Error::Error("connection reset by peer".into()));
                }
            }
        }
    }
    pub async fn write_frame(&mut self, frame: Frame) -> Result<(), Error>
    {
        // Serialize the frame
        let mut buf = serde_json::to_vec(&frame)?;

        // Prepend the length of the frame
        let len = buf.len() as u32;
        buf[0..4].copy_from_slice(&len.to_be_bytes());

        // Write the frame to the socket
        self.stream.write_all(&buf).await?;

        Ok(())
    }

    pub async fn handle_command(&self, request: Operation) -> Result<Response, Error> {
        match request {
            Operation::AddTriad(id, value) => {

                Ok(Response::TriadAdded(id))
            }
            Operation::RemoveTriad(id) => {
                self.state.triads.write().unwrap().remove(&id);
                Ok(Response::TriadRemoved(id))
            }
            Operation::AddWorkload(id, module) => {
                Ok(Response::WorkloadAdded(id))
            }
            Operation::RemoveWorkload(id) => {
                self.state.workloads.write().unwrap().remove(&id);
                Ok(Response::WorkloadRemoved(id))
            }
            Operation::RunWorkload(workload_id, triad_id) => {
                let workload = self.state.workloads.read().unwrap().get(&workload_id).unwrap();
                let triad = self.state.triads.read().unwrap().get(&triad_id).unwrap();
                Ok(Response::WorkloadRun(workload_id, triad_id))
            }
        }
    }
}