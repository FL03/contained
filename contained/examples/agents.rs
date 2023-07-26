extern crate contained;

use contained::agents::client::{AgentManager, Client};
use contained::agents::{Agent, Context, Stack, WasmEnv};
use contained::prelude::{AsyncResult, BoxedWasmValue, Shared};
use std::sync::{Arc, Mutex};
use tracing::instrument;
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
    tracing_subscriber::fmt::fmt()
        .compact()
        .with_line_number(false)
        .with_target(false)
        .init();
    // Initialize a new store
    let mut store = Store::default();
    // Initialize a new virtual environment
    let venv = CounterVenv::new(0);
    let imports = venv.imports(&mut store, None);

    let ctx = Context::new(Box::new(venv), Stack::new(), store);
    agents(Box::new([15.into()]), ctx, Some(imports)).await?;
    Ok(())
}

#[instrument(
    err,
    skip(ctx, imports),
    fields(function = "sample", module = "COUNTER_MODULE"),
    name = "example"
)]
async fn agents(
    args: BoxedWasmValue,
    ctx: Context,
    imports: Option<Imports>,
) -> AsyncResult<BoxedWasmValue> {
    let (tx, rx) = tokio::sync::mpsc::channel(100);
    let func = "sample";
    // Initialize a new agent; set the environment; then spawn it on a new thread
    let agent = Agent::new(rx, ctx);
    let mut client = Client::new(tx);
    agent.spawn(tokio::runtime::Handle::current());
    // Send the module to the agent
    let cid = client.include(COUNTER_MODULE.to_vec()).await?;
    // Execute the module
    let res = client
        .execute(cid.clone(), func.to_string(), args, imports)
        .await?;
    tracing::info!("Success: executed the function and got back {:?}", res);
    Ok(res)
}

pub fn counter_imports(env: &FunctionEnv<CounterVenv>, store: &mut Store) -> Imports {
    let get_counter_func = Function::new_typed_with_env(store, env, get_counter);
    let add_to_counter_func = Function::new_typed_with_env(store, env, add_to_counter);
    wasmer::imports! {
        "env" => {
            "get_counter" => get_counter_func,
            "add_to_counter" => add_to_counter_func,
        }
    }
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

impl WasmEnv for CounterVenv {
    fn imports(&self, store: &mut Store, with: Option<Imports>) -> Imports {
        let env = FunctionEnv::new(store, self.clone());
        let mut base = counter_imports(&env, store);
        if let Some(with) = with {
            base.extend(&with);
        }
        base
    }
}
