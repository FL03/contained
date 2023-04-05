extern crate contained_sdk as contained;

use contained::agents::{client::Client, Agent, VirtualEnv};
use contained::prelude::BoxedWasmValue;
use scsys::prelude::AsyncResult;
use wasmer::{wat2wasm, Store};
use wasmer::{Function, FunctionEnvMut};

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
    // Initialize a new store
    let store = Store::default();
    // Initialize a new virtual environment
    let venv = VirtualEnv::new(0);
    agents(Box::new([15.into()]), store, venv).await?;
    Ok(())
}

fn extra_imports(store: &mut Store, venv: VirtualEnv) -> wasmer::Imports {
    fn get_counter(env: FunctionEnvMut<VirtualEnv>) -> i32 {
        *env.data().value.lock().unwrap()
    }
    fn add_to_counter(env: FunctionEnvMut<VirtualEnv>, add: i32) -> i32 {
        let mut counter_ref = env.data().value.lock().unwrap();

        *counter_ref += add;
        *counter_ref
    }
    let env = venv.function_env(store);
    let get_counter_func = Function::new_typed_with_env(store, &env, get_counter);

    let add_to_counter_func = Function::new_typed_with_env(store, &env, add_to_counter);

    let with = wasmer::imports! {
        "env" => {
            "get_counter" => get_counter_func,
            "add_to_counter" => add_to_counter_func,
        }
    };
    venv.imports(store, Some(with))
}

async fn agents(
    args: BoxedWasmValue,
    mut store: Store,
    venv: VirtualEnv,
) -> AsyncResult<BoxedWasmValue> {
    let func = "sample";
    // Create a new imports object to be included with the provided venv
    let imports = extra_imports(&mut store, venv.clone());
    // Initialize a new agent; set the environment; then spawn it on a new thread
    let (agent, tx) = Agent::new(9);
    agent
        .set_environment(venv)
        .with_store(store)
        .spawn();
    // Initialize a new client
    let mut client = Client::new(tx);
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
