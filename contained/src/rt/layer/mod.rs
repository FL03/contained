/*
    Appellation: reqres <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{command::*, crud::*, event::*};

mod command;
mod crud;
mod event;

use tokio::sync::mpsc;

/// Type alias for the command receiver.
pub type CommandReceiver = mpsc::Receiver<Command>;
/// Type alias for the command sender.
pub type CommandSender = mpsc::Sender<Command>;
/// Type alias for the event receiver.
pub type SysEventReceiver = mpsc::Receiver<SystemEvent>;
/// Type alias for the event sender.
pub type SysEventSender = mpsc::Sender<SystemEvent>;
