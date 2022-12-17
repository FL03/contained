/*
    Appellation: primitives <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

///
pub type ChannelPackStd<T> = (std::sync::mpsc::Sender<T>, std::sync::mpsc::Receiver<T>);
///
pub type TokioChannelPackMPSC<T> = (mpsc::Sender<T>, mpsc::Receiver<T>);
///
pub type Locked<T> = Arc<Mutex<T>>;
