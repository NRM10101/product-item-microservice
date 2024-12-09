use tracing_subscriber;
use tracing::info;

pub fn init_tracing() {
    tracing_subscriber::fmt::init();
    info!("Tracing initialized.");
}
