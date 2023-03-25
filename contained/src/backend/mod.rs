pub use self::{context::*, settings::*};

pub(crate) mod context;
pub(crate) mod settings;

pub mod cli;

use crate::AsyncResult;
use std::sync::Arc;

pub struct Backend {
    ctx: Context,
}

impl Backend {
    pub fn new(ctx: Context) -> Self {
        Self { ctx }
    }
    pub fn context(&self) -> &Context {
        &self.ctx
    }
    pub async fn run(mut self) -> AsyncResult {
        loop {}
    }
    pub fn settings(&self) -> &Settings {
        self.ctx.settings()
    }
    pub fn setup(&self) {
        self.ctx.setup();
    }
    pub fn spawn(self) -> tokio::task::JoinHandle<AsyncResult> {
        tokio::spawn(self.run())
    }
}
