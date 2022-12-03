/*
    Appellation: Conduit <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{application::*, settings::*};

pub(crate) mod application;
pub(crate) mod settings;

pub mod cli;
pub mod states;

use std::sync::Arc;
use webrtc::api::{
    interceptor_registry::register_default_interceptors, media_engine::MediaEngine, APIBuilder,
};
use webrtc::ice_transport::ice_server::RTCIceServer;
use webrtc::interceptor::registry::Registry;
use webrtc::peer_connection::configuration::RTCConfiguration;
use webrtc::rtp_transceiver::rtp_codec::RTPCodecType;
use webrtc::track::track_local::track_local_static_rtp::TrackLocalStaticRTP;

#[tokio::main]
async fn main() -> scsys::BoxResult {
    let mut app = Application::default();
    println!("{:?}", &app);
    app.quickstart().await?;

    let wrt = WebRTCBuilder::default();
    let mut m = wrt.media()?;

    let mut registry = wrt.registry()?;

    // Create the API object with the MediaEngine
    let api = wrt.api()?;

    // Prepare the configuration
    let config = RTCConfiguration {
        ice_servers: vec![RTCIceServer {
            urls: vec!["stun:stun.l.google.com:19302".to_owned()],
            ..Default::default()
        }],
        ..Default::default()
    };

    // Create a new RTCPeerConnection
    let peer_connection = Arc::new(api.new_peer_connection(config).await?);

    // Allow us to receive 1 video track
    peer_connection
        .add_transceiver_from_kind(RTPCodecType::Video, &[])
        .await?;

    let (local_track_chan_tx, mut local_track_chan_rx) =
        tokio::sync::mpsc::channel::<Arc<TrackLocalStaticRTP>>(1);

    Ok(())
}

#[derive(Default)]
pub struct WebRTCBuilder;

impl WebRTCBuilder {
    pub fn api(&self) -> scsys::BoxResult<webrtc::api::API> {
        let api = APIBuilder::new()
            .with_media_engine(self.media()?)
            .with_interceptor_registry(self.registry()?)
            .build();
        Ok(api)
    }
    pub fn media(&self) -> scsys::BoxResult<MediaEngine> {
        let mut m = MediaEngine::default();
        m.register_default_codecs()?;
        Ok(m)
    }
    pub fn registry(&self) -> scsys::BoxResult<Registry> {
        let mut registry = Registry::default();
        registry = register_default_interceptors(registry, &mut self.media()?)?;
        Ok(registry)
    }
}
