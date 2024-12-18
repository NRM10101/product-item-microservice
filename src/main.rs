// main.rs
mod db;
mod entities;
mod handler;
mod logger;
mod repository;
mod router;
mod service;

use futures::executor::block_on;
use sea_orm::DbErr;
use std::net::SocketAddr;
use std::env;

async fn run() -> Result<(), DbErr> {
    tracing::info!("Starting the application...");

    let db = db::connect().await?; // Connect to the database
    tracing::info!("Database connection established.");

    let app = router::route::create_router(db); // Create the router

    // Run the server
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");
    let addr = SocketAddr::from(([0, 0, 0, 0], port)); // Listening on all interfaces
    tracing::info!("Listening on {}", addr);
    // let addr = SocketAddr::from(([0, 0, 0, 0], port));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    logger::init_tracing(); //we need to setup tracing-subscriber to capture and view the dubug log.

    if let Err(err) = block_on(run()) {
        tracing::error!("Application encountered an error: {}", err);
        panic!("{}", err);
    }

    tracing::info!("Application terminated successfully.");
}
