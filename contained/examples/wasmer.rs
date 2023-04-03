extern crate contained_sdk as contained;

use contained::prelude::Shared;
use contained::vm::{Client, Computer, VirtualEnv};
use scsys::prelude::AsyncResult;
use std::collections::HashMap;
use tokio::sync::mpsc;
use wasmer::{wat2wasm, Module, Store};

/// A sample Wasm module that exports a function called `increment`.
static COUNTER_MODULE: &[u8] = br#"
(module
    (func $get_counter (import "env" "get_counter") (result i32))
    (func $add_to_counter (import "env" "add_to_counter") (param i32) (result i32))
    (type $increment_t (func (param i32) (result i32)))
    (func $increment_f (type $increment_t) (param $x i32) (result i32)
      (block
        (loop
          (call $add_to_counter (i32.const 1))
          (set_local $x (i32.sub (get_local $x) (i32.const 1)))
          (br_if 1 (i32.eq (get_local $x) (i32.const 0)))
          (br 0)))
      call $get_counter)
    (export "increment" (func $increment_f)))
"#;

pub fn counter_module() -> std::borrow::Cow<'static, [u8]> {
    wat2wasm(COUNTER_MODULE).unwrap()
}

#[tokio::main]
async fn main() -> AsyncResult {
    // Initialize the tracing layer
    std::env::set_var("RUST_LOG", "info");
    tracing_subscriber::fmt::init();
    // Initialize a new store
    let store = Store::default();
    // Initialize a new module
    let module = Module::new(&store, counter_module())?;
    // Initialize new mpsc channels for sending and receiving modules
    let (tx_module, rx_module) = mpsc::channel(9);
    // Initialize new mpsc channels for sending and receiving results
    let (tx_result, rx_result) = mpsc::channel(9);
    // Initialize new mpsc channels for sending and receiving transformations
    let (tx_dirac, rx_dirac) = mpsc::channel(9);
    // Initialize a new computer; set the environment; then spawn it on a new thread
    Computer::new(rx_module, tx_result, rx_dirac)
        .set_environment(VirtualEnv::default())
        .spawn();
    // Initialize a new client
    let mut client = Client::new(tx_module, rx_result, tx_dirac);
    // Add a workload to the client
    client.add_workload(module).await?;
    // Cache the results of the computer
    let cache = client.cache_results().await?;
    // Assert that a single result was returned
    assert_eq!(cache.len(), 1);
    // Grab the result
    let res = {
        let key = cache.keys().next().unwrap();
        cache.get(key).unwrap()[0].i32().unwrap()
    };
    // Assert that the result is 5
    assert_eq!(res, 5);
    Ok(())
}

#[derive(Debug)]
pub struct Cluster {
    pub clients: HashMap<String, Shared<Client>>,
    pub computers: HashMap<String, Shared<Computer>>,
}

impl Cluster {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
            computers: HashMap::new(),
        }
    }
    pub fn add_computer(&mut self, id: String, computer: Shared<Computer>) {
        self.computers.insert(id, computer);
    }
    pub fn get_computer(&self, id: String) -> Option<Shared<Computer>> {
        self.computers.get(&id).cloned()
    }
}
