/*
    Appellation: events <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::network::*;

mod network;

#[derive(Debug)]
pub enum Event {
    Network(NetworkEvent),
}
