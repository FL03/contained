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

/// [Connect] describes a connection
#[derive(Debug)]
pub struct Connect {
    buf: BytesMut,
    stream: TcpStream,
}

impl Connect {
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
            // Attempt to parse a frame from the buffered data.
            if let Some(frame) = self.parse_frame()? {
                // returns when a sufficent amount of data has been buffered
                return Ok(Some(frame));
            }
            //
            if 0 == self.stream.read_buf(&mut self.buf).await? {
                if self.buf.is_empty() {
                    // The socket was closed cleanly; there was no data left in the buffer
                    return Ok(None);
                } else {
                    // The peer closed the socket while sending a frame.
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
