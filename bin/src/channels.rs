/*
    Appellation: channels <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::{Context, State};
use scsys::prelude::Locked;
use tokio::sync::{broadcast, mpsc, oneshot, watch};

pub type AsyncMutex<T = ()> = tokio::sync::Mutex<T>;

pub type BroadcastChannels<T = ()> = (broadcast::Sender<T>, broadcast::Receiver<T>);

pub type OneshotChannels<T = ()> = (oneshot::Sender<T>, oneshot::Receiver<T>);

pub type UnboundedMPSC<T = ()> = (mpsc::UnboundedSender<T>, mpsc::UnboundedReceiver<T>);

pub type WatchChannels<T = ()> = (watch::Sender<T>, watch::Receiver<T>);

pub enum Channel {
    Broadcast,
    MPSC,
    Oneshot,
    Unbounded,
}
