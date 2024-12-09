// // handlers.rs
// use axum::{
//     http::StatusCode,
//     response::Json,
//     routing::{get, post},
//     Router, Extension,
// };
// use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait};
// use serde_json::json;
// use super::entities::product;
// use super::entities::prelude::*;

// // Function to handle getting all products
// async fn get_all_products(Extension(db): Extension<DatabaseConnection>) -> Result<Json<Vec<product::Model>>, StatusCode> {
//     match super::query_all_products(&db).await {
//         Ok(products) => Ok(Json(products)),
//         Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
//     }
// }

// // Function to handle adding a product
// async fn add_product(Extension(db): Extension<DatabaseConnection>, Json(payload): Json<product::Model>) -> Result<StatusCode, StatusCode> {
//     let new_product = product::ActiveModel {
//         name: ActiveValue::Set(payload.name),
//         description: ActiveValue::Set(payload.description),
//         ..Default::default()
//     };
//     match Product::insert(new_product).exec(&db).await {
//         Ok(_) => Ok(StatusCode::CREATED),
//         Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
//     }
// }
