/*
    Appellation: connection <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: This module implements an explicit connection handler that supports the parsing of frames. The connection handler is used by the server and client to handle incoming connections.
        The primary motivation for this was to support operations on a custom frame
*/
use super::Frame;
use crate::core::Error;
use bytes::{Buf, BytesMut};
use std::io::Cursor;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

// Struct for handling incoming connections
pub struct Connection {
    buffer: BytesMut,
    stream: TcpStream,
}

impl Connection {
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
