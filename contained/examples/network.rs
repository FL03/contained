extern crate contained;

use contained::net::{
    node::{Backend, Context},
    NetResult,
};

#[tokio::main]
async fn main() -> NetResult {
    let ctx = Context::default();
    let backend = Backend::new();
    backend.start(ctx).await?;
    Ok(())
}
