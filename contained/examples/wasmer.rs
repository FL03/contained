extern crate contained_sdk as contained;

use contained::prelude::{Shared, State};
use std::borrow::Cow;
use std::sync::{Arc, Mutex};
use wasmer::{
    imports, wat2wasm, Function, FunctionEnv, FunctionEnvMut, Imports, Instance, Module, Store,
    TypedFunction,
};

/// A sample Wasm module that exports a function called `increment_counter_loop`.
static COUNTER_MODULE: &'static [u8] = br#"
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

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut app = Platform::new();
    // Compile the Wasm module.
    let module = Module::new(&app.store, counter_module())?;
    // Create an import object.
    let host = app.env.imports(&mut app.store);
    // Instantiate the module.
    let instance = Instance::new(&mut app.store, &module, &host)?;
    println!(
        "Original counter value: {:?}",
        *app.env.value.lock().unwrap()
    );
    // Here, we get a function called `increment_counter_loop` that was exported from the wasm module
    let increment_counter_loop: TypedFunction<i32, i32> = instance
        .exports
        .get_function("increment_counter_loop")?
        .typed(&mut app.store)?;
    // Let's call the `increment_counter_loop` exported function.
    let result = increment_counter_loop.call(&mut app.store, 5)?;
    // Grab the host counter value.
    let counter_value: i32 = *app.env.value.lock().unwrap();
    assert_eq!(counter_value, result);
    println!("Counter value (host): {:?}", counter_value);
    println!("Counter value (guest): {:?}", result);
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

pub struct Platform {
    env: Env,
    store: Store,
}

impl Platform {
    pub fn new() -> Self {
        Self {
            env: Env::default(),
            store: Store::default(),
        }
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
            id: scsys::prelude::BsonOid::new().to_hex(),
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
            id: scsys::prelude::BsonOid::new().to_hex(),
            state: Arc::new(Mutex::new(State::default())),
            value,
        }
    }
}
