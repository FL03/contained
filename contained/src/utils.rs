/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use decanter::prelude::{hasher, H256};
use wasmer::Module;

pub fn hash_module(module: Module) -> H256 {
    hasher(module.serialize().unwrap().as_ref()).into()
}
