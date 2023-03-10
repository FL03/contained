/*
    Appellation: build <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use clap::Args;
use serde::{Deserialize, Serialize};

#[derive(
    Args, Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct Build {
    /// Build a specific target
    #[clap(long, short)]
    pub package: Option<String>,
}

impl Build {
    pub fn cargo(&self, release: bool, workspace: bool) -> Vec<&str> {
        let mut args = vec!["build"];
        if let Some(pkg) = self.package.clone() {
            // args.push("-p");
            // args.push(&*pkg);
        } else if workspace {
            args.push("--workspace");
        }
        if release {
            args.push("--release");
        }
        args
    }
}