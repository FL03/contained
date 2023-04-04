extern crate contained_sdk as contained;

use contained::agents::{client::Client, Agent, VirtualEnv};
use decanter::prelude::hasher;
use scsys::prelude::AsyncResult;
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
    (export "sample" (func $increment_f)))
"#;

pub fn counter_module() -> std::borrow::Cow<'static, [u8]> {
    wat2wasm(COUNTER_MODULE).unwrap()
}

#[tokio::main]
async fn main() -> AsyncResult {
    // Initialize the tracing layer
    std::env::set_var("RUST_LOG", "info");
    tracing_subscriber::fmt::init();
    agents().await?;
    Ok(())
}

async fn agents() -> AsyncResult {
    // Initialize a new store
    let store = Store::default();
    // Initialize a new module
    let module = Module::new(&store, counter_module()).unwrap();
    // Initialize new mpsc channels for sending and receiving commands
    let (tx_cmd, rx_cmd) = mpsc::channel(9);
    // Initialize a new agent; set the environment; then spawn it on a new thread
    Agent::new(rx_cmd)
        .set_environment(VirtualEnv::default())
        .spawn();
    // Initialize a new client
    let mut client = Client::new(tx_cmd);
    // Send the module to the agent
    client.include(COUNTER_MODULE.to_vec()).await?;
    // Execute the module
    client.execute(
        hasher(module.clone().serialize()?).into(), 
        "sample".to_string(), 
        Box::new([15.into()])
    ).await?;
    Ok(())
}
