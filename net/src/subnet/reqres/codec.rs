/*
    Appellation: codec <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::{Proto, Request, Response};
use async_trait::async_trait;
use futures::{AsyncRead, AsyncWrite};
use libp2p::request_response::Codec;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ReqResCodec;

#[async_trait]
impl Codec for ReqResCodec {
    type Protocol = Proto;
    type Request = Request;
    type Response = Response;

    async fn read_request<T>(
        &mut self,
        _: &Self::Protocol,
        io: &mut T,
    ) -> Result<Self::Request, std::io::Error>
    where
        T: AsyncRead + Unpin + Send,
    {
        todo!()
    }

    async fn read_response<T>(
        &mut self,
        _: &Self::Protocol,
        io: &mut T,
    ) -> Result<Self::Response, std::io::Error>
    where
        T: AsyncRead + Unpin + Send,
    {
        todo!()
    }

    async fn write_request<T>(
        &mut self,
        _: &Self::Protocol,
        io: &mut T,
        req: Self::Request,
    ) -> Result<(), std::io::Error>
    where
        T: AsyncWrite + Unpin + Send,
    {
        todo!()
    }

    async fn write_response<T>(
        &mut self,
        _: &Self::Protocol,
        io: &mut T,
        res: Self::Response,
    ) -> Result<(), std::io::Error>
    where
        T: AsyncWrite + Unpin + Send,
    {
        todo!()
    }
}
