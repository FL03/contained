/*
    Appellation: rpc <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::{contexts::Context, sessions::Session, states::{Stateful, States}};
use scsys::{components::logging::Logger, prelude::BoxResult};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct RPCBackend {
    pub ctx: Context,
}

impl RPCBackend {
    pub fn new(ctx: Context) -> Self {
        Self { ctx }
    }
    pub fn setup_tracing(&self) -> BoxResult<&Self> {
        let name = self.ctx.settings.application.clone().unwrap_or_default().name;
        let tracer = opentelemetry_jaeger::new_agent_pipeline()
            .with_service_name(name.as_str())
            .with_max_packet_size(2usize.pow(13))
            .install_batch(opentelemetry::runtime::Tokio)?;
        tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::from_default_env())
            .with(tracing_subscriber::fmt::layer().with_span_events(FmtSpan::NEW | FmtSpan::CLOSE))
            .with(tracing_opentelemetry::layer().with_tracer(tracer))
            .try_init()?;
        Ok(self)
    }
    pub async fn run(&self) -> BoxResult<&Self> {
        samples::sample_client(self.ctx.clone()).await?;
        Ok(self)
    }
}
