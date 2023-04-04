/*
    Appellation: event <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::subnet;
use libp2p::request_response::ResponseChannel;
use scsys::prelude::AsyncResult;

#[derive(Debug)]
pub enum NetworkEvent {
    InboundRequest {
        request: String,
        channel: ResponseChannel<String>,
    },
}

impl NetworkEvent {
    pub async fn subnet_handle(&mut self, client: subnet::Client) -> AsyncResult {
        match self {
            Self::InboundRequest { request, channel } => {},
        }
        Ok(())
    }
    pub fn inbound_request(request: String, channel: ResponseChannel<String>) -> Self {
        Self::InboundRequest { request, channel }
    }
}
