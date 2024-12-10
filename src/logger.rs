use tracing_subscriber::EnvFilter;

pub fn init_tracing() {
    // Set up the tracing subscriber with environment variable filtering and formatted output
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env() // Allow filtering by environment variable
                .unwrap_or_else(|_| EnvFilter::new("info")), // Default to "info" level if env variable is not set
        )
        .with_target(true) // Include targets in the logs
        .with_thread_ids(true) // Include thread IDs in the logs
        .init();

    tracing::info!("Tracing initialized successfully.");
}
