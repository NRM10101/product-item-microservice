use crate::handler::{
    item_handler::{get_all_items, get_item_by_id,delete_item_by_id},
    product_handler::{delete_product_by_id, get_all_products, get_product_by_id},
};
use axum::{routing::{get,delete}, Extension, Router};
use sea_orm::DatabaseConnection;

pub fn create_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/products", get(get_all_products))
        .route("/products/:id", get(get_product_by_id))
        .route("/products/:id",delete(delete_product_by_id))
        // .route("/products", post(create_product)) // Attach handler
        .route("/items", get(get_all_items))
        .route("/items/:id", get(get_item_by_id))
        .route("/items/:id", delete(delete_item_by_id))
        .layer(Extension(db)) // Add database connection middleware
}
