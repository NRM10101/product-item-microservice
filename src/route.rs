use axum::{routing::get, Router};
use sea_orm::DatabaseConnection;
use axum::Extension;
use crate::handlers::*;

// Function to create and organize all application routes
pub fn create_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/products", get(get_all_products))
        .layer(Extension(db))
}
