extern crate contained;

use contained::net::{
    node::{Node, Context},
    NetResult,
};

#[tokio::main]
async fn main() -> NetResult {
    let ctx = Context::default();
    let backend = Node::new();
    backend.start(ctx).await?;
    Ok(())
}
