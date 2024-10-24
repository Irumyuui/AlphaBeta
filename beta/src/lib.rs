use tracing_appender::rolling;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};

pub mod common;
pub mod service;
pub mod web;

pub fn log_init() {
    if cfg!(debug_assertions) {
        tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer())
            .init();
        tracing::debug!("DEBUG VERSION");
    } else {
        let env_filter = EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into());
        let stdout_log = fmt::layer()
            .with_writer(std::io::stdout)
            .with_filter(env_filter);

        let env_filter = EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into());
        let file_appender = rolling::daily("logs", "app.log");
        let (non_blocking, _) = tracing_appender::non_blocking(file_appender);
        let file_log = fmt::layer()
            .with_writer(non_blocking)
            .with_filter(env_filter);

        tracing_subscriber::registry()
            .with(stdout_log)
            .with(file_log)
            .init();
    }
}
