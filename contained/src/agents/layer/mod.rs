/*
    Appellation: layer <agents>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: This module implements the async layer for the agents
*/
pub use self::{command::*, connect::*, frame::*};

mod command;
mod connect;
mod frame;

use scsys::prelude::AsyncResult;
use tokio::sync::oneshot;

pub type Responder<T = ()> = oneshot::Sender<AsyncResult<T>>;

#[async_trait::async_trait]
pub trait Connector {}
