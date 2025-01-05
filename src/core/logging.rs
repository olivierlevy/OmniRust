use tracing::{debug, error, info, span, warn, Level};
use tracing_subscriber::{fmt, EnvFilter};

/// Initializes the logger with the given filter level.
pub fn init_logger(level: &str) {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(level))
        .with_target(false)
        .with_timer(fmt::time::ChronoLocal::rfc3339())
        .init();
}

/// Demonstrates usage of the logger.
pub fn demo_logging() {
    let demo_span = span!(Level::INFO, "demo_span");
    let _enter = demo_span.enter();

    info!("Information log example");
    debug!("Debug log example");
    warn!("Warning log example");
    error!("Error log example");
}
