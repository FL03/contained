/*
    Appellation: connect <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::agents::layer::Frame;
use crate::prelude::Error;
use bytes::{Buf, BytesMut};
use std::io::Cursor;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, ToSocketAddrs};

// Struct for handling incoming connections
#[derive(Debug)]
pub struct Connection {
    buf: BytesMut,
    stream: TcpStream,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            // Allocate the buffer with 4kb of capacity.
            buf: BytesMut::with_capacity(4096),
            stream,
        }
    }
    pub async fn connect(addr: impl ToSocketAddrs) -> Result<Self, Error> {
        let stream = TcpStream::connect(addr).await?;
        Ok(Self::new(stream))
    }
    pub fn parse_frame(&mut self) -> Result<Option<Frame>, Error> {
        // Create the `T: Buf` type.
        let mut buf = Cursor::new(&self.buf[..]);

        // Check whether a full frame is available
        match Frame::check(&mut buf) {
            // An error was encountered
            Err(e) => match e {
                Error::Incomplete(_) => Ok(None),
                _ => Err(e),
            },
            Ok(_) => {
                // Get the byte length of the frame
                let len = buf.position() as usize;

                // Reset the internal cursor for the
                // call to `parse`.
                buf.set_position(0);

                // Parse the frame
                let frame = Frame::parse(&mut buf)?;

                // Discard the frame from the buffer
                self.buf.advance(len);

                // Return the frame to the caller.
                Ok(Some(frame))
            }
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
            if 0 == self.stream.read_buf(&mut self.buf).await? {
                // The remote closed the connection. For this to be
                // a clean shutdown, there should be no data in the
                // read buffer. If there is, this means that the
                // peer closed the socket while sending a frame.
                if self.buf.is_empty() {
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
