/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use std::path::{Path, PathBuf};

/// Remove the first and last charecters of a string
pub fn fnl_remove<T: Clone + ToString>(data: T) -> String {
    let data = data.to_string();
    let mut chars = data.chars();
    chars.next();
    chars.next_back();
    chars.as_str().to_string()
}
/// A simple function wrapper for 'discovering' the workspace project root
pub fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}
