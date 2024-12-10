use crate::handler::{
    item_handler::{
        delete_item_by_id,
        get_all_items,
        get_item_by_id,
        // update_item,
    },
    product_handler::{
        delete_product_by_id,
        // create_product,
        get_all_products,
        get_product_by_id,
    },
    // product_handler_create_update::create_product,
};
use axum::{
    routing::{
        delete,
        get,
        // put,
        // post,
    },
    Extension, Router,
};
use sea_orm::DatabaseConnection;

pub fn create_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/products", get(get_all_products))
        .route("/products/:id", get(get_product_by_id))
        .route("/products/:id", delete(delete_product_by_id))
        // .route("/products", post(create_product))


        .route("/items", get(get_all_items))
        .route("/items/:id", get(get_item_by_id))
        .route("/items/:id", delete(delete_item_by_id))
        // .route("/items/:id", put(update_item))
        // .route("/items", post(create_item)) // Create an item
        .layer(Extension(db)) 
}
