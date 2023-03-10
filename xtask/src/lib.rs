/*
    Appellation: xtask <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{commands::*, context::*, primitives::*, utils::*};

pub(crate) mod commands;
pub(crate) mod context;
pub(crate) mod utils;

pub mod cli;

///
#[macro_export]
macro_rules! cmd {
    ($(
        $x:expr;
        [ $( $y:expr ),* ]
    );*) => {
        {
            $(
                let mut cmd = std::process::Command::new($x);
                cmd.current_dir(scsys_xtask::project_root());
                let mut tmp = Vec::new();
                $(
                    tmp.push($y);
                )*
                cmd.args(tmp.as_slice()).status().expect("");
            )*
        }
    };
}

pub(crate) mod primitives {
    ///
    pub type Bundle<T = String> = std::collections::HashMap<T, Vec<Vec<T>>>;
}
