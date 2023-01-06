/*
   Appellation: routes <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/

pub mod assets;
pub mod index;

use axum::Router;

pub fn router() -> Router {
    Router::new().nest("/app", assets::router()).merge(index::router())
}
