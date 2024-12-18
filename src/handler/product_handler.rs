// use crate::entities::product::{self}; // Make sure to adjust this import based on your project's structure
use crate::service::product_service::{delete_product_by_id, get_all_products, get_product_by_id};
use axum::{
    body::Full,
    extract::{Extension, Json, Path},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sea_orm::DatabaseConnection;
use serde_json::to_vec;
use tracing::{error, info, warn}; // Serialize json data to Vec<u8>

use crate::entities::product::{CreateProductModel, UpdateProductModel};
use crate::service::product_service::{create_product, update_product};

pub async fn create_product_handler(
    Extension(db): Extension<DatabaseConnection>,
    Json(product_data): Json<CreateProductModel>,
) -> impl IntoResponse {
    info!("Handler: Creating a new product");
    match create_product(db, product_data).await {
        Ok(_) => {
            info!("Handler: Product creation successful");
            Response::builder()
                .status(StatusCode::CREATED)
                .body(Full::from("Product created"))
                .unwrap()
        }
        Err(err) => {
            error!("Handler: Failed to create product: {}", err);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(format!("Failed to create product: {}", err).into())
                .unwrap()
        }
    }
}

pub async fn update_product_handler(
    Extension(db): Extension<DatabaseConnection>,
    Json(product_data): Json<UpdateProductModel>,
) -> impl IntoResponse {
    info!("Handler: Updating a product");
    match update_product(db, product_data).await {
        Ok(_) => {
            info!("Handler: Product update successful");
            Response::builder()
                .status(StatusCode::OK)
                .body(Full::from("Product updated"))
                .unwrap()
        }
        Err(err) => {
            error!("Handler: Failed to update product: {}", err);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(format!("Failed to update product: {}", err).into())
                .unwrap()
        }
    }
}
pub async fn get_all_products_handler(
    Extension(db): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    info!("Handler: Fetching all products.");
    match get_all_products(&db).await {
        Ok(products) => {
            let serialized = to_vec(&products).unwrap();
            info!("Handler: Successfully retrieved all products.");
            Response::builder()
                .status(StatusCode::OK)
                .body(Full::from(serialized))
                .unwrap()
        }
        Err(e) => {
            error!("Handler: Failed to fetch products - {}", e);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Full::from("[]"))
                .unwrap()
        }
    }
}

pub async fn get_product_by_id_handler(
    Path(id): Path<i32>,
    Extension(db): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    info!("Handler: Fetching product by ID - {}", id);
    match get_product_by_id(&db, id).await {
        Ok(Some(product)) => {
            info!("Handler: Product found - ID {}", id);
            (StatusCode::OK, Json(product)).into_response()
        }
        Ok(None) => {
            warn!("Handler: Product not found - ID {}", id);
            (StatusCode::NOT_FOUND, "Product not found".to_string()).into_response()
        }
        Err(e) => {
            error!("Handler: Failed to retrieve product by ID {}: {}", id, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to retrieve product".to_string(),
            )
                .into_response()
        }
    }
}

pub async fn delete_product_by_id_handler(
    Path(id): Path<i32>,
    Extension(db): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    info!("Handler: Attempting to delete product by ID - {}", id);
    match delete_product_by_id(&db, id).await {
        Ok(_) => {
            info!("Handler: Product deleted successfully - ID {}", id);
            (StatusCode::NO_CONTENT).into_response()
        }
        Err(e) => {
            error!("Handler: Failed to delete product by ID {}: {}", id, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to delete product".to_string(),
            )
                .into_response()
        }
    }
}
