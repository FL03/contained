/*
    Appellation: conduit <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        def. A conduit is defined as a means of transporting or distributing something.

        For us, a conduit is a flexible node capable of assuming a number of different forms.
*/
use crate::peer::Peer;
use crate::{tokio_transport, BoxedTransport, Peerable};
use libp2p::{
    swarm::{NetworkBehaviour, Swarm},
    PeerId,
};

#[derive(Clone, Debug, Default)]
pub struct Conduit {
    peer: Peer,
}

impl Conduit {
    pub fn new(peer: Peer) -> Self {
        Self { peer }
    }
    pub fn peer(self) -> Peer {
        self.peer
    }
    pub fn swarm<B: NetworkBehaviour>(&self, behaviour: B) -> Swarm<B> {
        Swarm::with_tokio_executor(
            self.transport(),
            behaviour,
            PeerId::from(self.clone().peer().pk()),
        )
    }

    pub fn transport(&self) -> BoxedTransport {
        tokio_transport(&self.clone().peer().keypair(), true)
    }
}
