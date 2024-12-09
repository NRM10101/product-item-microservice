//handlers.rs
// use crate::entities::product;
use crate::entities::product::{self, CreateProductInput}; // Make sure to adjust this import based on your project's structure
use axum::{
    body::Full,
    extract::{Extension, Json, Path},
    http::StatusCode,
    response::{IntoResponse, Response},
};
// use sea_orm::{DatabaseConnection, EntityTrait};
use sea_orm::{ActiveValue::Set, DatabaseConnection, EntityTrait, ModelTrait};

use serde_json;
use serde_json::to_vec; // Serialize json data to Vec<u8>
                        //get all the products from the database
pub async fn get_all_products(Extension(db): Extension<DatabaseConnection>) -> impl IntoResponse {
    match query_all_products(&db).await {
        Ok(products) => {
            let serialized = to_vec(&products).unwrap(); // Serialize Vec<product::Model> to Vec<u8>
            Response::builder()
                .status(StatusCode::OK)
                .body(Full::from(serialized)) // Use Full with serialized data
                .unwrap()
        }
        Err(_) => {
            let empty_response: Vec<product::Model> = Vec::new();
            let serialized = to_vec(&empty_response).unwrap(); // Serialize empty Vec<product::Model> to Vec<u8>
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Full::from(serialized)) // Use Full with serialized data
                .unwrap()
        }
    }
}

async fn query_all_products(
    db: &DatabaseConnection,
) -> Result<Vec<product::Model>, sea_orm::DbErr> {
    product::Entity::find().all(db).await
}

//get product by id
pub async fn get_product_by_id(
    Path(id): Path<i32>,                          // Extract the ID from the URL path
    Extension(db): Extension<DatabaseConnection>, // Pass the database connection
) -> impl IntoResponse {
    // println!("{}", id);
    match query_product_by_id(&db, id).await {
        Ok(Some(product)) => (StatusCode::OK, Json(product)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Product not found".to_string()).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to retrieve product".to_string(),
        )
            .into_response(),
    }
}

async fn query_product_by_id(
    db: &DatabaseConnection,
    id: i32,
) -> Result<Option<product::Model>, sea_orm::DbErr> {
    product::Entity::find_by_id(id).one(db).await
}

//delete product by id
pub async fn delete_product_by_id(
    Path(id): Path<i32>,
    Extension(db): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    // Find the product by ID
    match product::Entity::find_by_id(id).one(&db).await {
        Ok(Some(product)) => {
            // Delete the product
            match product.delete(&db).await {
                Ok(_) => (StatusCode::NO_CONTENT).into_response(), // 204 No Content
                Err(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to delete product",
                )
                    .into_response(),
            }
        }
        Ok(None) => (StatusCode::NOT_FOUND, "Product not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

//create a product and stores in the database
pub async fn create_product(
    Json(input): Json<CreateProductInput>,
    Extension(db): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    match insert_product(&db, input).await {
        Ok(product) => {
            let serialized = serde_json::to_vec(&product).unwrap();
            Response::builder()
                .status(StatusCode::CREATED)
                .body(Full::from(serialized))
                .unwrap()
        }
        Err(_) => {
            let error_message = "Failed to create product";
            let serialized = serde_json::to_vec(&error_message).unwrap();
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(serialized.into())
                .unwrap()
        }
    }
}

async fn insert_product(
    db: &DatabaseConnection,
    input: CreateProductInput,
) -> Result<product::Model, sea_orm::DbErr> {
    let product = product::ActiveModel {
        name: Set(input.name),
        description: Set(input.description),
        ..Default::default()
    };

    let result = product::Entity::insert(product).exec(db).await?;

    product::Entity::find_by_id(result.last_insert_id)
        .one(db)
        .await?
        .ok_or(sea_orm::DbErr::Custom(
            "Failed to retrieve inserted product".to_string(),
        ))
}
