/*
    Appellation: commands <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::{Context, Settings};
use clap::{arg, command, value_parser, ArgAction, ArgMatches, Command};
use scsys::prelude::{AsyncResult, Contextual};
use std::{path::PathBuf, sync::Arc};
use tokio::task::JoinHandle;

pub async fn handle() -> JoinHandle<AsyncResult> {
    tokio::spawn(async move { Ok(()) })
}

#[derive(Clone, Debug)]
pub struct Runtime {
    pub ctx: Arc<Context>,
}

impl Runtime {
    pub fn new(ctx: Arc<Context>) -> Self {
        Self { ctx }
    }
    pub async fn handler(&self) -> AsyncResult<&Self> {
        if let Some(_up) = self.matches().get_one::<bool>("up") {
            let api = crate::api::from_context(self.context().clone());
            api.start().await?;
        }
        Ok(self)
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new(Arc::new(Context::default()))
    }
}

impl RuntimeCliSpec for Runtime {
    fn command(&self) -> Command {
        Command::new("command")
            .about("Select an availible actor to command")
            .arg(arg!(service: -s --service <SERVICE>).action(ArgAction::SetTrue))
    }
}

impl Contextual for Runtime {
    type Cnf = Settings;

    type Ctx = Context;

    fn context(&self) -> &Self::Ctx {
        self.ctx.as_ref()
    }
}

impl From<Arc<Context>> for Runtime {
    fn from(ctx: Arc<Context>) -> Self {
        Self::new(ctx)
    }
}

impl From<Context> for Runtime {
    fn from(ctx: Context) -> Self {
        Self::from(Arc::new(ctx))
    }
}

impl std::fmt::Display for Runtime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::json!({"ctx": self.ctx.as_ref()}))
    }
}

pub trait RuntimeCliSpec {
    fn base(&self, sc: Command) -> ArgMatches {
        command!()
            .subcommand(sc)
            .arg(
                arg!(
                    -c --config <FILE> "Sets a custom config file"
                )
                // We don't have syntax yet for optional options, so manually calling `required`
                .required(false)
                .value_parser(value_parser!(PathBuf))
                .default_value("/config/Conduit.toml"),
            )
            .arg(
                arg!(debug: -d --debug)
                    .action(ArgAction::SetTrue)
                    .help("Optionally startup the debugger"),
            )
            .arg(
                arg!(release: -r --release)
                    .action(ArgAction::SetTrue)
                    .help("Optionally startup application in release mode"),
            )
            .arg(
                arg!(up: -u --up)
                    .action(ArgAction::SetTrue)
                    .help("Signals for a system to turn on"),
            )
            .arg(arg!(verbose: -v --verbose).action(ArgAction::SetTrue))
            .propagate_version(true)
            .subcommand_required(false)
            .arg_required_else_help(true)
            .get_matches()
    }
    fn command(&self) -> Command;
    fn matches(&self) -> ArgMatches {
        self.base(self.command())
    }
}
