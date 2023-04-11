/*
    Appellation: primitives <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{constants::*, types::*};

mod constants {
    /// Default address for [libp2p::Multiaddr]
    pub const DEFAULT_MULTIADDR: &str = "/ip4/0.0.0.0/tcp/0";

    pub const DEFAULT_MAINNET_ADDR: &str = "/ip4/0.0.0.0/tcp/9001";

    pub const DEFAULT_SUBNET_ADDR: &str = "/ip4/0.0.0.0/tcp/9099";
}

mod types {
    use crate::NetworkError;
    use libp2p::core::{muxing::StreamMuxerBox, transport::Boxed};
    pub use libp2p::kad::QueryId;
    pub use libp2p::{Multiaddr, PeerId};

    /// Type alias for a [Boxed] two-tuple, ([PeerId], [StreamMuxerBox])
    pub type BoxedTransport = Boxed<(PeerId, StreamMuxerBox)>;
    /// Type alias for a [Result] for a given type with a [NetError]
    pub type NetworkResult<T = ()> = Result<T, NetworkError>;
}
