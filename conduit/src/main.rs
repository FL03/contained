/*
    Appellation: Conduit <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{application::*, settings::*};

pub(crate) mod application;
pub(crate) mod settings;

pub mod cli;

#[tokio::main]
async fn main() -> scsys::BoxResult {
    let mut app = Application::default();
    println!("{:?}", &app);
    app.quickstart().await?;

    Ok(())
}
