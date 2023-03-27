use std::sync::{Arc, Mutex};
use wasmer::{Instance, Module, Store, Function, FunctionEnv, FunctionEnvMut, imports, TypedFunction, wat2wasm};

pub type Sharded<T> = Arc<Mutex<T>>;
pub type ShardedCounter = Sharded<i32>;

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


fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Let's declare the Wasm module.
    let wasm_bytes = wat2wasm(
        br#"
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
"#,
    )?;
    // Create a store, that holds the engine.
    let mut store = Store::default();

    println!("Compiling module...");
    // Compile the Wasm module.
    let module = Module::new(&store, wasm_bytes)?;
    // Create an environment to share data between host and guest.
    let env = Env::new(0);
    let func_env = env.function_env(&mut store);

    let get_counter_func = Function::new_typed_with_env(
        &mut store, 
        &func_env, 
        get_counter
    );
    
    let add_to_counter_func = Function::new_typed_with_env(
        &mut store, 
        &func_env, 
        add_to_counter
    );
    
    let import_object = imports! {
        "env" => {
            "get_counter" => get_counter_func,
            "add_to_counter" => add_to_counter_func,
        }
    };

    println!("Instantiating module...");
    // Let's instantiate the Wasm module.
    let instance = Instance::new(&mut store, &module, &import_object)?;

    // Here we go.
    //
    // The Wasm module exports a function called `increment_counter_loop`. Let's get it.
    let increment_counter_loop: TypedFunction<i32, i32> = instance
        .exports
        .get_function("increment_counter_loop")?
        .typed(&mut store)?;

    let counter_value: i32 = *env.value.lock().unwrap();
    println!("Initial ounter value: {:?}", counter_value);

    println!("Calling `increment_counter_loop` function...");
    // Let's call the `increment_counter_loop` exported function.
    //
    // It will loop five times thus incrementing our counter five times.
    let result = increment_counter_loop.call(&mut store, 5)?;

    let counter_value: i32 = *env.value.lock().unwrap();
    println!("New counter value (host): {:?}", counter_value);
    assert_eq!(counter_value, 5);

    println!("New counter value (guest): {:?}", result);
    assert_eq!(result, 5);

    Ok(())
}




#[derive(Clone)]
pub struct Env {
    pub value: Arc<Mutex<i32>>,
}

impl Env {
    pub fn new(value: i32) -> Self {
        Self {
            value: Arc::new(Mutex::new(value)),
        }
    }
    pub fn function_env(&self, store: &mut Store) -> FunctionEnv<Self> {
        FunctionEnv::new(store,self.clone())
    }
}

impl Default for Env {
    fn default() -> Self {
        Self {
            value: Arc::new(Mutex::new(0)),
        }
    }
}

impl From<Sharded<i32>> for Env {
    fn from(value: Sharded<i32>) -> Self {
        Self { value }
    }
}

fn get_counter(env: FunctionEnvMut<Env>) -> i32 {
    *env.data().value.lock().unwrap()
}

fn add_to_counter(env: FunctionEnvMut<Env>, add: i32) -> i32 {
    let mut counter_ref = env.data().value.lock().unwrap();

    *counter_ref += add;
    *counter_ref
}
