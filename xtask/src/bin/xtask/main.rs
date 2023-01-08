/*
    Appellation: xtask <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use anyhow::Result;
use clap::{arg, command, value_parser, ArgAction, ArgMatches, Command};
use std::path::PathBuf;
use xtask::{auto, builder, setup};

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("Welcome to xtask...");

    let matches = cli();

    let release = *matches.get_one::<bool>("release").unwrap_or(&false);
    let workspace = *matches.get_one::<bool>("workspace").unwrap_or(&false);

    match matches.subcommand() {
        Some(("action", sub_matches)) => {
            if sub_matches.get_one::<bool>("auto").is_some() {
                auto()?;
            }
            if sub_matches.get_one::<bool>("build").is_some() {
                builder(release, workspace)?;
            }
            if sub_matches.get_one::<bool>("setup").is_some() {
                setup(true, false)?;
            }
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }

    Ok(())
}

pub fn actions() -> Command {
    Command::new("action")
        .arg(arg!(auto: -a --auto).action(ArgAction::SetTrue))
        .arg(arg!(build: -b --build).action(ArgAction::SetTrue))
        .arg(arg!(setup: -s --setup <PLATFORM>).action(ArgAction::SetTrue))
}

pub fn cli() -> ArgMatches {
    command!()
        .subcommand(actions())
        .arg(
            arg!(
                -c --config <FILE> "Sets a custom config file"
            )
            // We don't have syntax yet for optional options, so manually calling `required`
            .required(false)
            .value_parser(value_parser!(PathBuf))
            .default_value("/xtask.yml"),
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
        .arg(arg!(workspace: -w --workspace).action(ArgAction::SetTrue))
        .propagate_version(true)
        .subcommand_required(false)
        .arg_required_else_help(true)
        .get_matches()
}
