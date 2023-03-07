extern crate contained;

use contained::net::{
    backend::{Backend, Context},
    NetResult,
};

#[tokio::main]
async fn main() -> NetResult {
    let ctx = Context::default();
    let backend = Backend::new();
    backend.start(ctx).await?;
    Ok(())
}
