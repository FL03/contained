/*
   Appellation: assets <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use axum::{
    body::{boxed, Body, BoxBody},
    routing::get,
    Router,
};
use http::{Request, Response};
use hyper::{StatusCode, Uri};
use tower::util::ServiceExt;
use tower_http::services::ServeDir;

pub fn router() -> Router {
    Router::new().route("/", get(wasm_handler))
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
