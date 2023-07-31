/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use decanter::prelude::{hasher, H256};
use wasmer::Module;

/// [hash_module] is a simple utility function that takes a [Module] and returns a [H256] hash.
pub fn hash_module(module: &Module) -> H256 {
    hasher(module.serialize().unwrap().as_ref()).into()
}
