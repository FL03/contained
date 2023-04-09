/*
    Appellation: connect <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::Frame;
use crate::prelude::{Error, Resultant};
use bytes::{Buf, BytesMut};
use std::io::Cursor;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};
use tokio::net::{TcpStream, ToSocketAddrs};

/// [Connect] describes a connection
#[derive(Debug)]
pub struct Connect {
    buf: BytesMut,
    stream: BufWriter<TcpStream>,
}

impl Connect {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            // Allocate the buffer with 4kb of capacity.
            buf: BytesMut::with_capacity(4096),
            stream: BufWriter::new(stream),
        }
    }
    pub async fn connect(addr: impl ToSocketAddrs) -> Resultant<Self> {
        let stream = TcpStream::connect(addr).await?;
        Ok(Self::new(stream))
    }
    fn parse_frame(&mut self) -> Resultant<Option<Frame>> {
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
    pub async fn read_frame(&mut self) -> Resultant<Option<Frame>> {
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
    /// Write a decimal frame to the stream
    async fn write_decimal(&mut self, val: u64) -> Resultant {
        use std::io::Write;

        // Convert the value to a string
        let mut buf = [0u8; 12];
        let mut buf = Cursor::new(&mut buf[..]);
        write!(&mut buf, "{}", val)?;

        let pos = buf.position() as usize;
        self.stream.write_all(&buf.get_ref()[..pos]).await?;
        self.stream.write_all(b"\r\n").await?;

        Ok(())
    }
    pub async fn write_frame(&mut self, frame: &Frame) -> Resultant {
        // Serialize the frame
        let mut buf = serde_json::to_vec(frame)?;

        // Prepend the length of the frame
        let len = buf.len() as u32;
        buf[0..4].copy_from_slice(&len.to_be_bytes());

        match frame {
            Frame::Content { cid, content } => {
                self.stream.write_all(&buf).await?;
            }
            _ => {
                self.stream.write_all(&buf).await?;
            }
        }

        self.stream.flush().await?;
        Ok(())
    }
    /// Write a frame literal to the stream
    async fn write_value(&mut self, frame: &Frame) -> Resultant {
        match frame {
            Frame::Error(val) => {
                self.stream.write_u8(b'-').await?;
                self.stream.write_all(val.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            _ => unreachable!("unimplemented"),
        }

        Ok(())
    }
}
