/*
    Appellation: event <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use libp2p::request_response::ResponseChannel;

#[derive(Debug)]
pub enum ClientEvent {
    InboundRequest {
        request: String,
        channel: ResponseChannel<String>,
    },
}
