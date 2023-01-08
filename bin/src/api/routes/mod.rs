/*
   Appellation: routes <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/

pub mod assets;
pub mod index;

pub fn api() -> axum::Router {
    axum::Router::new()
        .merge(index::router())
        .nest("/app", assets::router())
}
