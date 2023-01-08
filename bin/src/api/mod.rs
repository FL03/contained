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
    Api::new(ctx)
}

pub async fn handle(ctx: crate::Context) -> tokio::task::JoinHandle<Api> {
    tokio::spawn(async move {
        let api = std::sync::Arc::new(from_context(ctx));
        api.start().await.expect("");
        api.as_ref().clone()
    })
}

pub(crate) mod interface {
    use crate::{api::routes, Context};
    use acme::prelude::{
        servers::{Server, ServerSpec},
        WebBackend,
    };
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

    #[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
    pub struct Api {
        pub ctx: Context,
        pub server: Server,
    }

    impl Api {
        pub fn new(ctx: Context) -> Self {
            let server = Server::from(ctx.cnf.server.pieces());
            Self { ctx, server }
        }
        /// Quickstart the server with the outlined client
        pub async fn start(&self) -> AsyncResult {
            self.server().serve(self.client().await).await
        }
    }

    impl std::fmt::Display for Api {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", serde_json::to_string(&self).ok().unwrap())
        }
    }

    #[async_trait::async_trait]
    impl WebBackend for Api {
        type Ctx = Context;

        type Server = Server;

        async fn client(&self) -> axum::Router {
            Router::new()
                .merge(routes::api())
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
                .layer(axum::Extension(self.ctx.clone()))
        }

        fn context(&self) -> Self::Ctx {
            self.ctx.clone()
        }

        fn server(&self) -> Self::Server {
            self.server.clone()
        }
    }
}
