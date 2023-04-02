use contained_music as music;

#[cfg(test)]
use music::neo::triads::*;
use tokio::sync::mpsc;
use wasmer::{wat2wasm, Module, Store};

/// A sample Wasm module that exports a function called `increment_counter_loop`.
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
    (export "increment_counter_loop" (func $increment_f)))
"#;

pub fn counter_module() -> std::borrow::Cow<'static, [u8]> {
    wat2wasm(COUNTER_MODULE).unwrap()
}
#[tokio::test]
async fn test_tonic() {
    let store = Store::default();
    // Compile the Wasm module.
    let module = Module::new(&store, counter_module()).unwrap();
    // Initialize new mpsc channels for sending and receiving modules
    let (tx_module, rx_module) = mpsc::channel(9);
    tx_module.send(module.clone()).await.unwrap();
    // Initialize a new tonic
    let tonic = {
        // Initialize a new triad
        let triad = Triad::new(0.into(), TriadClass::Major);
        // Initialize the channel manager for the tonic
        let channels = TonicChannel::new(rx_module, mpsc::channel(9).1);
        Tonic::new(channels, triad)
    };
    // Spawn the tonic
    tonic.spawn().await.unwrap().expect("");
    tokio::signal::ctrl_c().await.unwrap();
}
