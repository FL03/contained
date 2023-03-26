/*
    Appellation: contained <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, RwLock};
use std::thread;

use tokio::io::{AsyncWriteExt, AsyncReadExt};
use wasmer::{Instance, Module, Store};

use tokio::net::TcpStream as AsyncTcpStream;
use tokio::sync::{mpsc, Mutex};



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Ok(())
}


// Struct for storing the state of a triadic structure
#[derive(Clone)]
struct Surface {
    id: u32,
    value: i32,
}

// Struct for storing the state of a workload
struct Workload {
    id: u32,
    module: Module,
}

// Struct for storing the state of the application
struct AppState {
    triads: RwLock<HashMap<u32, Instance>>,
    workloads: RwLock<HashMap<u32, Workload>>,
}

pub enum Frame {
    Triad(u32, i32),
    Workload(u32, u32),
}

pub enum Message {
    Frame(Frame),
    Shutdown,
}

// Struct for handling incoming connections
struct ConnectionHandler {
    app_state: Arc<AppState>,
}

impl ConnectionHandler {
    async fn handle_connection(&self, mut stream: AsyncTcpStream) -> anyhow::Result<()> {
        let mut buffer = [0; 1024];
        let mut data = String::new();

        loop {
            let bytes_read = stream.read(&mut buffer).await?;
            if bytes_read == 0 {
                break;
            }
            data.push_str(&String::from_utf8_lossy(&buffer[..bytes_read]));
        }

        let response = self.process_request(data.trim()).await?;

        stream.write_all(response.as_bytes()).await?;
        stream.shutdown().await?;

        Ok(())
    }

    async fn process_request(&self, request: &str) -> anyhow::Result<String> {
        let tokens: Vec<&str> = request.split_whitespace().collect();

        match tokens[0] {
            "add-triad" => {
                let id = tokens[1].parse::<u32>()?;
                let value = tokens[2].parse::<i32>()?;
                // self.app_state
                //     .triads
                //     .write()
                //     .unwrap()
                //     .insert(id, Instance::new(&mut Module::from_file(&Store::default(), "triad.wasm")?, )?);
                Ok(format!("Triad {} added", id))
            }
            "remove-triad" => {
                let id = tokens[1].parse::<u32>()?;
                self.app_state.triads.write().unwrap().remove(&id);
                Ok(format!("Triad {} removed", id))
            }
            "add-workload" => {
                let id = tokens[1].parse::<u32>()?;
                let module = Module::from_file(&Store::default(), tokens[2])?;
                self.app_state
                    .workloads
                    .write()
                    .unwrap()
                    .insert(id, Workload { id, module });
                Ok(format!("Workload {} added", id))
            }
            "remove-workload" => {
                let id = tokens[1].parse::<u32>()?;
                self.app_state.workloads.write().unwrap().remove(&id);
                Ok(format!("Workload {} removed", id))
            }
            "run-workload" => {
                let workload_id = tokens[1].parse::<u32>()?;
                let triad_id = tokens[2].parse::<u32>()?;
                let triad = self.app_state.triads.read().unwrap().get(&triad_id).unwrap();
                let mut workload = self.app_state.workloads.read().unwrap().get(&workload_id).unwrap();
                Ok(format!("Workload {} ran sucessfully", workload_id))
            }
            _ => Ok("Unknown command".to_string()),
        }
    }
}