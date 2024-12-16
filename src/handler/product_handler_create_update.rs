// use axum::{
//     body::Body,
//     http::{Response, StatusCode},
//     response::IntoResponse,
//     Extension,
//     Json,
// };
// use sea_orm::{entity::*, DatabaseConnection,Set, TransactionTrait};
// use tracing::{info, error};

// use crate::entities::product;
// use crate::entities::product::{UpdateProductModel,CreateProductModel};

// // Function to handle the creation of a product
// pub async fn create_product(
//     Extension(db): Extension<DatabaseConnection>,
//     Json(product_data): Json<CreateProductModel>,
// ) -> impl IntoResponse {
//     info!("Creating a new product...");

//     let product_model = product::ActiveModel {
//         name: Set(product_data.name.to_owned()),
//         description: Set(Some(product_data.description.to_owned())),
//         ..Default::default()  // Ensure all other fields are set to their default if any
//     };

//     // Insert the new product into the database
//     match product::Entity::insert(product_model).exec(&db).await {
//         Ok(_result) => {
//             info!("Product successfully created");
//             Response::builder()
//                 .status(StatusCode::CREATED)
//                 .body(Body::from("Product created"))
//                 .unwrap()
//         }
//         Err(err) => {
//             error!("Failed to create product: {:?}", err);
//             Response::builder()
//                 .status(StatusCode::INTERNAL_SERVER_ERROR)
//                 .body(Body::from("Failed to create product"))
//                 .unwrap()
//         }
//     }
// }

// pub async fn update_product(
//     Extension(db): Extension<DatabaseConnection>,
//     Json(product_data): Json<UpdateProductModel>,
// ) -> impl IntoResponse {
//     info!("Updating a product...");

//     let mut transaction = db.begin().await.unwrap();

//     match product::Entity::find_by_id(product_data.id).one(&mut transaction).await {
//         Ok(Some(product_model)) => {
//             let mut active_model = product_model.into_active_model();

//             if let Some(name) = product_data.name {
//                 active_model.name = Set(name);
//             }
//             if let Some(description) = product_data.description {
//                 active_model.description = Set(Some(description));
//             }

//             match active_model.update(&mut transaction).await {
//                 Ok(_) => {
//                     transaction.commit().await.unwrap();
//                     info!("Product successfully updated");
//                     Response::builder()
//                         .status(StatusCode::OK)
//                         .body(Body::from("Product updated"))
//                         .unwrap()
//                 },
//                 Err(err) => {
//                     error!("Failed to update product: {:?}", err);
//                     transaction.rollback().await.unwrap();
//                     Response::builder()
//                         .status(StatusCode::INTERNAL_SERVER_ERROR)
//                         .body(Body::from("Failed to update product"))
//                         .unwrap()
//                 }
//             }
//         },
//         Ok(None) => {
//             error!("Product not found");
//             Response::builder()
//                 .status(StatusCode::NOT_FOUND)
//                 .body(Body::from("Product not found"))
//                 .unwrap()
//         },
//         Err(err) => {
//             error!("Database error: {:?}", err);
//             Response::builder()
//                 .status(StatusCode::INTERNAL_SERVER_ERROR)
//                 .body(Body::from("Database error"))
//                 .unwrap()
//         }
//     }
// }