/*
    Appellation: subnet <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use contained_sdk as contained;

#[tokio::main]
async fn main() -> scsys::prelude::AsyncResult {
    let settings = contained::backend::Settings::build()?;
    println!("Settings: {:?}", settings);
    Ok(())
}
