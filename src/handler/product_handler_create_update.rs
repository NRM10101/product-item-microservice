// use axum::{
//     body::Body,
//     http::StatusCode,
//     response::Response,
//     Extension, Json,
// };
// use futures::future::{BoxFuture, FutureExt};
// use sea_orm::{DatabaseConnection, entity::*};
// use serde_json::to_vec;

// use crate::entities::{prelude::Product, product::CreateProductInput};

// // Function to handle the creation of a product
// pub fn create_product(
//     Json(input): Json<CreateProductInput>,
//     Extension(db): Extension<DatabaseConnection>,
// ) -> BoxFuture<'static, Result<Response, (StatusCode, String)>> {
//     async move {
//         match insert_product(&db, input).await {
//             Ok(product) => {
//                 let serialized = to_vec(&product).unwrap();
//                 Ok(Response::builder()
//                     .status(StatusCode::CREATED)
//                     .body(Body::from(serialized))
//                     .unwrap())
//             }
//             Err(e) => Err((
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 format!("Failed to create product: {}", e),
//             )),
//         }
//     }
//     .boxed()
// }

// async fn insert_product(
//     db: &DatabaseConnection,
//     input: CreateProductInput,
// ) -> Result<Product::Model, sea_orm::DbErr> {
//     let active_model = Product::ActiveModel {
//         name: Set(input.name),
//         description: Set(input.description),
//         ..Default::default()
//     };

//     Product::insert(active_model)
//         .exec(db)
//         .await?
//         .model()
//         .ok_or(sea_orm::DbErr::Custom("Failed to fetch inserted model".to_string()))
// }
