/*
    Appellation: interface <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::{contexts::Context, sessions::Session, states::{Stateful, States}};
use scsys::{components::logging::Logger, prelude::BoxResult};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Application<T: Stateful> {
    pub ctx: Context,
    pub session: Session,
    pub state: States<T>,
    
}

impl<T: Stateful> Application<T> {
    pub fn new(ctx: Context) -> Self {
        let session = Session::default();
        let state = Default::default();
        Self { ctx, session, state }
    }
    pub fn setup_logger(&self) -> &Self {
        match &self.ctx.settings.logger {
            Some(v) => v.setup(),
            None => Logger::from("info").setup(),
        }
        self
    }
    pub fn set_state(&mut self, state: States<T>) -> &Self {
        self.state = state;
        self
    }
    pub async fn spawn_rpc(&self) -> BoxResult<&Self> {
        crate::rpc::RPCBackend::new(self.ctx.clone())
            .run()
            .await?;
        Ok(self)
    }
    pub async fn run(&self) -> BoxResult<&Self> {
        self.setup_logger();

        self.spawn_rpc().await?;
        // self.spawn_rpc().await?;

        Ok(self)
    }
}

impl<T: Stateful> std::fmt::Display for Application<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}
