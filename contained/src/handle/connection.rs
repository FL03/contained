/*
    Appellation: connection <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::core::Error;

use bytes::{Buf, BytesMut};
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Frame {
    Empty,
    Triad { id: u32, value: i32 },
    Workload { id: u32, module: u32 },
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
                let (id, module) = serde_json::from_slice::<(u32, u32)>(&data)?;

                Ok(Self::Workload { id, module })
            }
            _ => Err(Error::InvalidType),
        }
    }
}

// Struct for handling incoming connections
pub struct ConnectionHandler {
    buffer: BytesMut,
    stream: TcpStream,
}

impl ConnectionHandler {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            // Allocate the buffer with 4kb of capacity.
            buffer: BytesMut::with_capacity(4096),
            stream,
        }
    }
    pub fn parse_frame(&mut self) -> Result<Option<Frame>, Error> {
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
            // An error was encountered
            Err(e) => match e {
                Error::Incomplete(_) => Ok(None),
                _ => Err(e),
            },
        }
    }
    pub async fn read_frame(&mut self) -> Result<Option<Frame>, Error> {
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
    pub async fn write_frame(&mut self, frame: Frame) -> Result<(), Error> {
        // Serialize the frame
        let mut buf = serde_json::to_vec(&frame)?;

        // Prepend the length of the frame
        let len = buf.len() as u32;
        buf[0..4].copy_from_slice(&len.to_be_bytes());

        // Write the frame to the socket
        self.stream.write_all(&buf).await?;

        Ok(())
    }
}
