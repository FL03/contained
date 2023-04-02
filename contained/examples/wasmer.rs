extern crate contained_sdk as contained;

use contained::prelude::{Shared, State};
use scsys::prelude::{AsyncResult, BsonOid};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use wasmer::{imports, wat2wasm, Imports, Instance, Module, Store};
use wasmer::{Function, FunctionEnv, FunctionEnvMut, TypedFunction};

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

pub type BoxedWasmValue = Box<[wasmer::Value]>;

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
    // Initialize a new computer; set the environment; then spawn it on a new thread
    Computer::new(rx_module, tx_result)
        .set_environment(Env::default())
        .spawn();
    // Initialize a new client
    let mut client = Client::new(tx_module, rx_result, mpsc::channel(9).0);
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

fn get_counter(env: FunctionEnvMut<Env>) -> i32 {
    *env.data().value.lock().unwrap()
}

fn add_to_counter(env: FunctionEnvMut<Env>, add: i32) -> i32 {
    let mut counter_ref = env.data().value.lock().unwrap();

    *counter_ref += add;
    *counter_ref
}

#[derive(Debug)]
pub struct Cluster {
    pub cache: HashMap<String, BoxedWasmValue>,
    pub computers: HashMap<String, Shared<Computer>>,
}

impl Cluster {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
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

pub struct Client {
    pub cache: HashMap<String, BoxedWasmValue>,
    pub program: mpsc::Sender<Module>,
    pub results: mpsc::Receiver<BoxedWasmValue>,
    pub transform: mpsc::Sender<String>,
}

impl Client {
    pub fn new(
        program: mpsc::Sender<Module>,
        results: mpsc::Receiver<BoxedWasmValue>,
        transform: mpsc::Sender<String>,
    ) -> Self {
        Self {
            cache: HashMap::new(),
            program,
            results,
            transform,
        }
    }
    pub async fn add_workload(&mut self, module: Module) -> AsyncResult {
        self.program.send(module).await?;
        Ok(())
    }
    pub async fn cache_results(&mut self) -> AsyncResult<&HashMap<String, BoxedWasmValue>> {
        while let Some(res) = self.results.recv().await {
            self.cache.insert(BsonOid::new().to_hex(), res);
        }
        Ok(&self.cache)
    }
}

#[derive(Debug)]
pub struct Computer {
    env: Arc<Mutex<Env>>,
    program: mpsc::Receiver<Module>,
    results: mpsc::Sender<BoxedWasmValue>,
    store: Store,

    transform: mpsc::Receiver<String>,
}

impl Computer {
    pub fn new(program: mpsc::Receiver<Module>, results: mpsc::Sender<BoxedWasmValue>) -> Self {
        Self {
            env: Arc::new(Mutex::new(Env::default())),
            program,
            results,
            store: Store::default(),
            transform: mpsc::channel(9).1,
        }
    }
    pub async fn run(mut self) -> AsyncResult {
        Ok(loop {
            tokio::select! {
                Some(module) = self.program.recv() => {
                    tracing::info!("Received a new program");
                    let host = self.env.lock().unwrap().imports(&mut self.store);
                    tracing::info!("Instantiating module with the imported host functions");
                    let instance = Instance::new(&mut self.store, &module, &host).expect("Failed to instantiate module");
                    tracing::info!("Success: Instantiated module with the imported host functions");
                    let run: TypedFunction<i32, i32> = instance.exports.get_function("increment_counter_loop")?.typed(&mut self.store)?;
                    tracing::info!("Success: Got the counter function from the module");
                    let result = run.call(&mut self.store, 5)?;
                    tracing::info!("Success: Ran the counter function from the module\n\tCounter value (host): {:?}", result);
                    self.results.send(Box::new([result.into()])).await?;
                }
                Some(transform) = self.transform.recv() => {
                    println!("{:?}", transform);
                }
                _ = tokio::signal::ctrl_c() => {
                    tracing::warn!("Signal received, shutting down");
                    break;
                }
                else => tracing::warn!("Tonic has no more work to do"),
            }
        })
    }
    pub fn set_environment(mut self, env: Env) -> Self {
        self.env = Arc::new(Mutex::new(env));
        self
    }
    pub fn spawn(self) -> tokio::task::JoinHandle<AsyncResult> {
        tokio::spawn(self.run())
    }
}

#[derive(Clone, Debug)]
pub struct Env {
    pub state: Shared<State>,
    pub value: Shared<i32>,
}

impl Env {
    pub fn new(value: i32) -> Self {
        Self {
            state: Arc::new(Mutex::new(State::default())),
            value: Arc::new(Mutex::new(value)),
        }
    }
    pub fn function_env(&self, store: &mut Store) -> FunctionEnv<Self> {
        FunctionEnv::new(store, self.clone())
    }
    pub fn imports(&self, store: &mut Store) -> Imports {
        let env = self.function_env(store);
        let get_counter_func = Function::new_typed_with_env(store, &env, get_counter);

        let add_to_counter_func = Function::new_typed_with_env(store, &env, add_to_counter);

        imports! {
            "env" => {
                "get_counter" => get_counter_func,
                "add_to_counter" => add_to_counter_func,
            }
        }
    }
}

impl Default for Env {
    fn default() -> Self {
        Self::new(0)
    }
}

impl From<Shared<i32>> for Env {
    fn from(value: Shared<i32>) -> Self {
        Self {
            state: Arc::new(Mutex::new(State::default())),
            value,
        }
    }
}
