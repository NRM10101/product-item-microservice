use crate::entities::item::{CreateItemModel, UpdateItemModel};
use crate::service::item_service::{create_item, update_item};
use axum::{
    body::{Body, Full},
    extract::{Extension, Json, Path},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sea_orm::DatabaseConnection;
use serde_json::to_vec; // Serialize json data to Vec<u8>
use tracing::{error, info, warn};

// use crate::service::item_service::query_all_items;

//-------------------------------------------------

//get all the products from the database
// pub async fn get_all_items(Extension(db): Extension<DatabaseConnection>) -> impl IntoResponse {
//     info!("Attempting to fetch all items from the database.");
//     match query_all_items(&db).await {
//         Ok(items) => {
//             info!("Successfully retrieved all items.");
//             let serialized = to_vec(&items).unwrap(); // Serialize Vec<product::Model> to Vec<u8>
//             Response::builder()
//                 .status(StatusCode::OK)
//                 .body(Full::from(serialized)) // Use Full with serialized data
//                 .unwrap()
//         }
//         Err(e) => {
//             error!("Failed to fetch items: {:?}", e);
//             let empty_response: Vec<item::Model> = Vec::new();
//             let serialized = to_vec(&empty_response).unwrap(); // Serialize empty Vec<product::Model> to Vec<u8>
//             Response::builder()
//                 .status(StatusCode::INTERNAL_SERVER_ERROR)
//                 .body(Full::from(serialized)) // Use Full with serialized data
//                 .unwrap()
//         }
//     }
// }

// // async fn query_all_items(db: &DatabaseConnection) -> Result<Vec<item::Model>, sea_orm::DbErr> {
// //     item::Entity::find().all(db).await
// // }

// pub async fn get_item_by_id(
//     Path(id): Path<i32>,                          // Extract the ID from the URL path
//     Extension(db): Extension<DatabaseConnection>, // Pass the database connection
// ) -> impl IntoResponse {
//     // println!("{}", id);
//     info!("Fetching item by ID: {}", id);
//     match query_item_by_id(&db, id).await {
//         Ok(Some(item)) => {
//             info!("Item found: ID {}", id);
//             (StatusCode::OK, Json(item)).into_response()
//         }
//         Ok(None) => {
//             warn!("Item not found: ID {}", id);
//             (StatusCode::NOT_FOUND, "Item not found".to_string()).into_response()
//         }
//         Err(e) => {
//             error!("Failed to retrieve item by ID {}: {}", id, e);
//             (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 "Failed to retrieve item".to_string(),
//             )
//                 .into_response()
//         }
//     }
// }

// async fn query_item_by_id(
//     db: &DatabaseConnection,
//     id: i32,
// ) -> Result<Option<item::Model>, sea_orm::DbErr> {
//     item::Entity::find_by_id(id).one(db).await
// }

// // Delete item by ID  --------use ModelTrait from seaorm for this
// pub async fn delete_item_by_id(
//     Path(id): Path<i32>,
//     Extension(db): Extension<DatabaseConnection>,
// ) -> impl IntoResponse {
//     // Find the item by ID
//     info!("Attempting to delete item by ID: {}", id);
//     match item::Entity::find_by_id(id).one(&db).await {
//         Ok(Some(item)) => {
//             // Delete the item
//             match item.delete(&db).await {
//                 Ok(_) => {
//                     info!("Successfully deleted item: ID {}", id);
//                     (StatusCode::NO_CONTENT).into_response()
//                 } // 204 No Content
//                 Err(e) => {
//                     error!("Failed to delete item by ID {}: {}", id, e);
//                     (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete item").into_response()
//                 }
//             }
//         }
//         Ok(None) => {
//             warn!("No item found to delete: ID {}", id);
//             (StatusCode::NOT_FOUND, "Item not found").into_response()
//         }
//         Err(e) => {
//             error!("Database error during deletion of item by ID {}: {}", id, e);
//             (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
//         }
//     }
// }

//--------------

use crate::service::item_service::{delete_item_by_id, get_all_items, get_item_by_id};
pub async fn get_all_items_handler(
    Extension(db): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    info!("Handler: Fetching all items");
    match get_all_items(&db).await {
        Ok(items) => {
            let serialized = to_vec(&items).unwrap();
            info!("Handler: Successfully retrieved all items");
            Response::builder()
                .status(StatusCode::OK)
                .body(Full::from(serialized))
                .unwrap()
        }
        Err(e) => {
            error!("Handler: Failed to fetch items - {}", e);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Full::from("[]"))
                .unwrap()
        }
    }
}

pub async fn get_item_by_id_handler(
    Path(id): Path<i32>,
    Extension(db): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    info!("Handler: Fetching item by ID - {}", id);
    match get_item_by_id(&db, id).await {
        Ok(Some(item)) => {
            info!("Handler: Item found - ID {}", id);
            (StatusCode::OK, Json(item)).into_response()
        }
        Ok(None) => {
            warn!("Handler: Item not found - ID {}", id);
            (StatusCode::NOT_FOUND, "Item not found".to_string()).into_response()
        }
        Err(e) => {
            error!("Handler: Failed to retrieve item by ID {}: {}", id, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to retrieve item".to_string(),
            )
                .into_response()
        }
    }
}

pub async fn delete_item_by_id_handler(
    Path(id): Path<i32>,
    Extension(db): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    info!("Handler: Deleting item by ID - {}", id);
    match delete_item_by_id(&db, id).await {
        Ok(_) => {
            info!("Handler: Successfully deleted item - ID {}", id);
            (StatusCode::NO_CONTENT).into_response()
        }
        Err(e) => {
            error!("Handler: Failed to delete item by ID {}: {}", id, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to delete item".to_string(),
            )
                .into_response()
        }
    }
}

pub async fn create_item_handler(
    Extension(db): Extension<DatabaseConnection>,
    Json(data): Json<CreateItemModel>,
) -> impl IntoResponse {
    info!("Creating a new Item...");
    match create_item(db, data).await {
        Ok(_) => {
            info!("Item successfully created");
            Response::builder()
                .status(StatusCode::CREATED)
                .body(Body::from("Item created"))
                .unwrap();
        }
        Err(err) => {
            error!("Failed to create Item: {:?}", err);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(format!("Failed to create item: {}", err)))
                .unwrap();
        }
    }
}

pub async fn update_item_handler(
    Extension(db): Extension<DatabaseConnection>,
    Json(data): Json<UpdateItemModel>,
) -> impl IntoResponse {
    info!("Updating an Item...");
    match update_item(db, data).await {
        Ok(_) => {
            info!("Item successfully updated");
            Response::builder()
                .status(StatusCode::OK)
                .body(Body::from("Item updated"))
                .unwrap()
        }
        Err(err) => {
            error!("Failed to update Item: {:?}", err);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(format!("Failed to update item: {}", err)))
                .unwrap()
        }
    }
}
