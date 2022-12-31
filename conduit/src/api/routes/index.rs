/*
   Appellation: index <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use crate::Context;
use axum::{body::{boxed, Body, BoxBody}, extract::Path, routing::get, Extension, Json, Router};
use http::{Response, Request};
use hyper::{StatusCode, Uri};
use scsys::prelude::Message;
use serde_json::{json, Value};
use tower::util::ServiceExt;
use tower_http::services::ServeDir;

pub fn router() -> Router {
    Router::new()
        .route("/", get(landing))
        .route("/app/:uri", get(wasm_handler))
        .route("/settings", get(settings))
        .route("/notifications/:id", get(notifications).post(notifications))
}

/// Define the landing endpoint
pub async fn landing() -> Json<Value> {
    let msg = Message::from("welcome to flow");
    Json(json!(msg))
}

/// Implements a notification endpoint
pub async fn notifications(Path(id): Path<usize>) -> Json<Value> {
    let data = json!({ "id": id });
    Json(json!(Message::from(data)))
}

/// Broadcasts the current settings specified by the user for the interface and other technical systems to leverage
pub async fn settings(Extension(ctx): Extension<Context>) -> Json<Value> {
    Json(json!(ctx.cnf))
}

///
pub async fn wasm_handler(uri: Uri) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let dir = "../wasm/pkg";
    let res = static_assets(dir, uri.clone()).await?;

    if res.status() == StatusCode::NOT_FOUND {
        // try with `.html`
        // TODO: handle if the Uri has query parameters
        match format!("{}.html", uri).parse() {
            Ok(uri_html) => static_assets(dir, uri_html).await,
            Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Invalid URI".to_string())),
        }
    } else {
        Ok(res)
    }
}

/// Fetch some static assets from a given directory
async fn static_assets(dir: &str, uri: Uri) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let req = Request::builder().uri(uri).body(Body::empty()).unwrap();

    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    match ServeDir::new(dir).oneshot(req).await {
        Ok(res) => Ok(res.map(boxed)),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", err),
        )),
    }
 }
