extern crate contained_sdk as contained;

use contained::prelude::{Shared, State};
use scsys::prelude::BsonOid;
use std::sync::{Arc, Mutex};
use std::{borrow::Cow, collections::HashMap};
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

pub fn counter_module() -> Cow<'static, [u8]> {
    wat2wasm(COUNTER_MODULE).unwrap()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let store = Store::default();
    // Compile the Wasm module.
    let module = Module::new(&store, counter_module())?;

    let mut runtime = Runtime::new();
    runtime.add_env("env-1".to_string(), Env::new(0));
    runtime.add_env("env-2".to_string(), Env::new(1));
    runtime.add_workload("counter_module".to_string(), module);
    assert_eq!(
        runtime.run("env-1".to_string(), "counter_module".to_string())?,
        5
    );
    assert_eq!(
        runtime.run("env-2".to_string(), "counter_module".to_string())?,
        6
    );
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

pub struct Stack {
    pub envs: HashMap<String, Env>,
    pub workloads: HashMap<String, Module>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            envs: HashMap::new(),
            workloads: HashMap::new(),
        }
    }
    pub fn add_env(&mut self, id: String, env: Env) {
        self.envs.insert(id, env);
    }
    pub fn add_workload(&mut self, id: String, workload: Module) {
        self.workloads.insert(id, workload);
    }
}

pub struct Driver {
    pub env: Env,
    pub workload: Module,
}

impl Driver {
    pub fn new(env: Env, workload: Module) -> Self {
        Self { env, workload }
    }
    pub fn instance(&self, store: &mut Store) -> Instance {
        let host = self.env.imports(store);
        Instance::new(store, &self.workload, &host).expect("Failed to instantiate module")
    }
}

pub struct Runtime {
    pub stack: Stack,
    pub store: Store,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            stack: Stack::new(),
            store: Store::default(),
        }
    }
    pub fn add_env(&mut self, id: String, env: Env) {
        self.stack.envs.insert(id, env);
    }
    pub fn add_workload(&mut self, id: String, workload: Module) {
        self.stack.workloads.insert(id, workload);
    }
    pub fn get_env(&self, id: String) -> Option<Env> {
        self.stack.envs.get(&id).cloned()
    }
    pub fn get_workload(&self, id: String) -> Option<Module> {
        self.stack.workloads.get(&id).cloned()
    }

    pub fn run(
        &mut self,
        space: String,
        workload: String,
    ) -> Result<i32, Box<dyn std::error::Error + Send + Sync>> {
        let env = self.get_env(space).unwrap();
        let workload = self.get_workload(workload).unwrap();
        let exec = Driver::new(env, workload);
        let increment_counter_loop: TypedFunction<i32, i32> = exec
            .instance(&mut self.store)
            .exports
            .get_function("increment_counter_loop")?
            .typed(&mut self.store)?;
        let result = increment_counter_loop.call(&mut self.store, 5)?;
        println!("Counter value (host): {:?}", result);
        Ok(result)
    }
}

#[derive(Clone)]
pub struct Env {
    pub id: String,
    pub state: Shared<State>,
    pub value: Shared<i32>,
}

impl Env {
    pub fn new(value: i32) -> Self {
        Self {
            id: BsonOid::new().to_hex(),
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
            id: BsonOid::new().to_hex(),
            state: Arc::new(Mutex::new(State::default())),
            value,
        }
    }
}
