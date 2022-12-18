/*
    Appellation: scsys-xtask <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use std::{
    path::{Path, PathBuf},
    process::Command,
};

///
pub fn command(program: &str, args: Vec<&str>) -> anyhow::Result<()> {
    let mut cmd = Command::new(program);
    cmd.current_dir(project_root());
    cmd.args(args.as_slice()).status()?;
    Ok(())
}

/// Fetch the project root unless specified otherwise with a CARGO_MANIFEST_DIR env variable
pub fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}

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
