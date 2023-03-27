use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use wasmer::{imports, wat2wasm, Function, Instance, Module, Store};

use tokio::sync::mpsc;

pub static COUNTER_MODULE: &'static [u8] = br#"
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
    (export "increment_counter_loop" (func $increment_f)))
"#;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Define the number of WASM instances to create
    let num_instances = 3;

    // Create a module and store for the WASM instances
    let store = Store::default();
    let module = Module::new(&store, wat2wasm(COUNTER_MODULE).unwrap()).unwrap();

    // Create a channel to send messages to the WASM tasks
    let (tx, mut rx) = mpsc::channel::<Message>(num_instances);

    Ok(())
}

pub struct Message {
    pub id: usize,
    pub value: i32,
}

// Define a custom environment for the WASM module
struct Env {
    counter: Arc<Mutex<u32>>,
}

impl Env {
    fn new() -> Self {
        Env {
            counter: Arc::new(Mutex::new(0)),
        }
    }
}
