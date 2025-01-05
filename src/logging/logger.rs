use tracing::{info, error};
use tracing_subscriber::fmt;

pub fn init_logger() {
    fmt().init();
}

pub fn log_info(message: &str) {
    info!("{}", message);
}

pub fn log_error(message: &str) {
    error!("{}", message);
}
