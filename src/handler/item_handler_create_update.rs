// use axum::{
//     body::Body,
//     http::{Response, StatusCode},
//     response::IntoResponse,
//     Extension, Json,
// };
// use sea_orm::DatabaseConnection;
// // use sea_orm::QueryFilter;
// // use sea_orm::{entity::*, ActiveModelTrait, DatabaseConnection, Set};
// use tracing::{error, info};

// use crate::entities::item::{CreateItemModel, UpdateItemModel};
// use crate::service::item_service::{create_item, update_item};
// // use crate::entities::item;


// // Function to handle the creation of a product
// pub async fn create_item(
//     Extension(db): Extension<DatabaseConnection>,
//     Json(data): Json<CreateItemModel>,
// ) -> impl IntoResponse {
//     info!("Creating a new Item...");

//     let item_model = item::ActiveModel {
//         product_id: Set(data.product_id),
//         name: Set(data.name.clone()),
//         price: Set(data.price.clone()),
//         ..Default::default() // All other fields set to their default values
//     };

//     match item::Entity::insert(item_model).exec(&db).await {
//         Ok(_result) => {
//             info!("Item successfully created");
//             Response::builder()
//                 .status(StatusCode::CREATED)
//                 .body(Body::from("Item created"))
//                 .unwrap()
//         }
//         Err(err) => {
//             error!("Failed to create Item: {:?}", err);
//             Response::builder()
//                 .status(StatusCode::INTERNAL_SERVER_ERROR)
//                 .body(Body::from("Failed to create Item"))
//                 .unwrap()
//         }
//     }
// }

// // Function to handle updating an existing item
// pub async fn update_item(
//     Extension(db): Extension<DatabaseConnection>,
//     Json(data): Json<UpdateItemModel>,
// ) -> impl IntoResponse {
//     info!("Updating an Item...");

//     let result = item::Entity::find_by_id(data.id).one(&db).await;

//     match result {
//         Ok(Some(item_model)) => {
//             let mut am = item_model.into_active_model();

//             if let Some(product_id) = data.product_id {
//                 am.product_id = Set(product_id);
//             }
//             if let Some(name) = data.name.clone() {
//                 am.name = Set(name);
//             }
//             if let Some(price) = data.price.clone() {
//                 am.price = Set(price);
//             }

//             match am.update(&db).await {
//                 Ok(_updated_model) => {
//                     info!("Item successfully updated");
//                     Response::builder()
//                         .status(StatusCode::OK)
//                         .body(Body::from("Item updated"))
//                         .unwrap()
//                 }
//                 Err(err) => {
//                     error!("Failed to update Item: {:?}", err);
//                     Response::builder()
//                         .status(StatusCode::INTERNAL_SERVER_ERROR)
//                         .body(Body::from("Failed to update Item"))
//                         .unwrap()
//                 }
//             }
//         }
//         Ok(None) => {
//             error!("Item not found");
//             Response::builder()
//                 .status(StatusCode::NOT_FOUND)
//                 .body(Body::from("Item not found"))
//                 .unwrap()
//         }
//         Err(err) => {
//             error!("Database error: {:?}", err);
//             Response::builder()
//                 .status(StatusCode::INTERNAL_SERVER_ERROR)
//                 .body(Body::from("Database error"))
//                 .unwrap()
//         }
//     }
// }
