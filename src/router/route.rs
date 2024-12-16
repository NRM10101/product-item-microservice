use crate::handler::{
    item_handler::{
        create_item_handler, delete_item_by_id_handler, get_all_items_handler,
        get_item_by_id_handler, update_item_handler,
    },
    product_handler::{
        create_product_handler, delete_product_by_id_handler, get_all_products_handler,
        get_product_by_id_handler, update_product_handler,
    },
};
use axum::{
    routing::{delete, get, post, put},
    Extension, Router,
};
use sea_orm::DatabaseConnection;

pub fn create_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/products", get(get_all_products_handler))
        .route("/products/:id", get(get_product_by_id_handler))
        .route("/products/:id", delete(delete_product_by_id_handler))
        .route("/products", post(create_product_handler))
        .route("/products/:id", put(update_product_handler))
        
        .route("/items", get(get_all_items_handler))
        .route("/items/:id", get(get_item_by_id_handler))
        .route("/items/:id", delete(delete_item_by_id_handler))
        .route("/items", post(create_item_handler)) // Create an item
        .route("/items/:id", put(update_item_handler))
        .layer(Extension(db))
}
