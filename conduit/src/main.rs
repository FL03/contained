/*
    Appellation: Conduit <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{application::*, settings::*};

pub(crate) mod application;
pub(crate) mod settings;

#[tokio::main]
async fn main() -> scsys::BoxResult {
    let mut app = Application::default();
    println!("{:?}", &app);
    app.with_logging();

    Ok(())
}
