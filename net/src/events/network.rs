/*
    Appellation: event <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::subnet::{
    self,
    proto::reqres::{Request, Response},
};
use libp2p::request_response::ResponseChannel;
use scsys::prelude::AsyncResult;

#[derive(Debug)]
pub enum NetworkEvent {
    InboundRequest {
        request: Request,
        channel: ResponseChannel<Response>,
    },
}

impl NetworkEvent {
    pub fn inbound_request(request: Request, channel: ResponseChannel<Response>) -> Self {
        Self::InboundRequest { request, channel }
    }
}
