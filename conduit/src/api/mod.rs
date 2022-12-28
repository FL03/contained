/*
   Appellation: api <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
pub use self::interface::*;

pub mod routes;

pub fn new() -> Api {
    Api::default()
}

pub fn from_context(ctx: crate::Context) -> Api {
    Api::new(ctx.clone(), ctx.cnf.server.port)
}

pub(crate) mod interface {
    use crate::{api::routes, Context};
    use acme::servers::{Server, ServerSpec};
    use axum::Router;
    use http::header::{HeaderName, AUTHORIZATION};
    use scsys::AsyncResult;
    use serde::{Deserialize, Serialize};
    use tower_http::{
        compression::CompressionLayer,
        propagate_header::PropagateHeaderLayer,
        sensitive_headers::SetSensitiveHeadersLayer,
        trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    };

    #[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
    pub struct Api {
        pub ctx: Context,
        pub server: Server,
    }

    impl Api {
        pub fn new(ctx: Context, port: u16) -> Self {
            let server = Server::from(port);
            Self { ctx, server }
        }
        pub async fn client(&self) -> Router {
            let mut router = Router::new();
            // Merge other routers into the base router
            router = router.merge(routes::index::router());
            router = router
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(DefaultMakeSpan::new().include_headers(true))
                        .on_request(DefaultOnRequest::new().level(tracing::Level::INFO))
                        .on_response(DefaultOnResponse::new().level(tracing::Level::INFO)),
                )
                .layer(SetSensitiveHeadersLayer::new(std::iter::once(
                    AUTHORIZATION,
                )))
                .layer(CompressionLayer::new())
                .layer(PropagateHeaderLayer::new(HeaderName::from_static(
                    "x-request-id",
                )))
                .layer(axum::Extension(self.ctx.clone()));
            router
        }
        /// Returns an owned instance of the server
        pub fn server(&self) -> &Server {
            &self.server
        }
        /// Quickstart the server with the outlined client
        pub async fn serve(&self) -> AsyncResult {
            self.server().serve(self.client().await).await
        }
    }

    impl std::fmt::Display for Api {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", serde_json::to_string(&self).ok().unwrap())
        }
    }
}
