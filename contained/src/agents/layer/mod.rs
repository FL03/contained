/*
    Appellation: layer <agents>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: This module implements the async layer for the agents
*/
pub use self::{command::*, event::*};

mod command;
mod event;

use crate::prelude::AsyncResult;
use tokio::sync::oneshot;

pub type OneshotSender<T = ()> = oneshot::Sender<AsyncResult<T>>;
