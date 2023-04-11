extern crate contained_sdk as contained;

use contained::agents::{client::AgentManager, Agent, WasmVenv};
use contained::prelude::{BoxedWasmValue, Shared};
use scsys::prelude::AsyncResult;
use std::sync::{Arc, Mutex};
use wasmer::{wat2wasm, Imports, Store};
use wasmer::{Function, FunctionEnv, FunctionEnvMut};

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

fn get_counter(env: FunctionEnvMut<CounterVenv>) -> i32 {
    *env.data().value.lock().unwrap()
}
fn add_to_counter(env: FunctionEnvMut<CounterVenv>, add: i32) -> i32 {
    let mut counter_ref = env.data().value.lock().unwrap();

    *counter_ref += add;
    *counter_ref
}

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
    // Initialize a new virtual environment
    let venv = CounterVenv::new(0);
    agents(Box::new([15.into()]), store, venv).await?;
    Ok(())
}

async fn agents(
    args: BoxedWasmValue,
    mut store: Store,
    venv: CounterVenv,
) -> AsyncResult<BoxedWasmValue> {
    let func = "sample";
    // Create a new imports object to be included with the provided venv
    let imports = venv.imports(&mut store, None);
    // Initialize a new agent; set the environment; then spawn it on a new thread
    let (agent, mut client) = Agent::new(9, Box::new(venv));
    agent
        .with_store(store)
        .spawn(tokio::runtime::Handle::current());
    // Send the module to the agent
    let cid = client.include(COUNTER_MODULE.to_vec()).await?;
    // Execute the module
    let res = client
        .execute(cid.clone(), func.to_string(), args, Some(imports))
        .await?;
    tracing::info!(
        "Success: used the module ({}) to execute the '{}' function and returned {:?}",
        cid,
        func,
        res
    );
    Ok(res)
}

#[derive(Clone, Debug)]
pub struct CounterVenv {
    pub value: Shared<i32>,
}

impl CounterVenv {
    pub fn new(value: i32) -> Self {
        Self {
            value: Arc::new(Mutex::new(value)),
        }
    }
}

impl Default for CounterVenv {
    fn default() -> Self {
        Self::new(0)
    }
}

impl WasmVenv for CounterVenv {
    fn imports(&self, store: &mut Store, with: Option<Imports>) -> Imports {
        let env = FunctionEnv::new(store, self.clone());
        let get_counter_func = Function::new_typed_with_env(store, &env, get_counter);

        let add_to_counter_func = Function::new_typed_with_env(store, &env, add_to_counter);

        let mut base = wasmer::imports! {
            "env" => {
                "get_counter" => get_counter_func,
                "add_to_counter" => add_to_counter_func,
            }
        };
        if let Some(with) = with {
            base.extend(&with);
        }
        base
    }
}
