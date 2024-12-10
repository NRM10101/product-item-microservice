use crate::entities::item::{
    self,
    // UpdateItemInput
}; // Make sure to adjust this import based on your project's structure
use axum::{
    body::Full,
    extract::{Extension, Json, Path},
    http::StatusCode,
    response::{IntoResponse, Response},
};
// use sea_orm::{DatabaseConnection, EntityTrait};
use sea_orm::{
    // ActiveModelTrait, ActiveValue::Set,
    DatabaseConnection,
    EntityTrait,
    //  IntoActiveModel,
    ModelTrait,
};
use serde_json::to_vec; // Serialize json data to Vec<u8>
use tracing::{error, info, warn};

//get all the products from the database
pub async fn get_all_items(Extension(db): Extension<DatabaseConnection>) -> impl IntoResponse {
    info!("Attempting to fetch all items from the database.");
    match query_all_items(&db).await {
        Ok(items) => {
            info!("Successfully retrieved all items.");
            let serialized = to_vec(&items).unwrap(); // Serialize Vec<product::Model> to Vec<u8>
            Response::builder()
                .status(StatusCode::OK)
                .body(Full::from(serialized)) // Use Full with serialized data
                .unwrap()
        }
        Err(e) => {
            error!("Failed to fetch items: {:?}", e);
            let empty_response: Vec<item::Model> = Vec::new();
            let serialized = to_vec(&empty_response).unwrap(); // Serialize empty Vec<product::Model> to Vec<u8>
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Full::from(serialized)) // Use Full with serialized data
                .unwrap()
        }
    }
}

async fn query_all_items(db: &DatabaseConnection) -> Result<Vec<item::Model>, sea_orm::DbErr> {
    item::Entity::find().all(db).await
}

pub async fn get_item_by_id(
    Path(id): Path<i32>,                          // Extract the ID from the URL path
    Extension(db): Extension<DatabaseConnection>, // Pass the database connection
) -> impl IntoResponse {
    // println!("{}", id);
    info!("Fetching item by ID: {}", id);
    match query_item_by_id(&db, id).await {
        Ok(Some(item)) => {
            info!("Item found: ID {}", id);
            (StatusCode::OK, Json(item)).into_response()
        }
        Ok(None) => {
            warn!("Item not found: ID {}", id);
            (StatusCode::NOT_FOUND, "Item not found".to_string()).into_response()
        }
        Err(e) => {
            error!("Failed to retrieve item by ID {}: {}", id, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to retrieve item".to_string(),
            )
                .into_response()
        }
    }
}

async fn query_item_by_id(
    db: &DatabaseConnection,
    id: i32,
) -> Result<Option<item::Model>, sea_orm::DbErr> {
    item::Entity::find_by_id(id).one(db).await
}

// Delete item by ID  --------use ModelTrait from seaorm for this
pub async fn delete_item_by_id(
    Path(id): Path<i32>,
    Extension(db): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    // Find the item by ID
    info!("Attempting to delete item by ID: {}", id);
    match item::Entity::find_by_id(id).one(&db).await {
        Ok(Some(item)) => {
            // Delete the item
            match item.delete(&db).await {
                Ok(_) => {
                    info!("Successfully deleted item: ID {}", id);
                    (StatusCode::NO_CONTENT).into_response()
                } // 204 No Content
                Err(e) => {
                    error!("Failed to delete item by ID {}: {}", id, e);
                    (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete item").into_response()
                }
            }
        }
        Ok(None) => {
            warn!("No item found to delete: ID {}", id);
            (StatusCode::NOT_FOUND, "Item not found").into_response()
        }
        Err(e) => {
            error!("Database error during deletion of item by ID {}: {}", id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
        }
    }
}

//Update an Item
// pub async fn update_item(
//     //IntoActiveModel,IntoActiveModel
//     Path(id): Path<i32>,
//     Json(input): Json<UpdateItemInput>,
//     Extension(db): Extension<DatabaseConnection>,
// ) -> impl IntoResponse {
//     match item::Entity::find_by_id(id).one(&db).await {
//         Ok(Some(existing_item)) => {
//             let mut active_item = existing_item.into_active_model(); // Convert to ActiveModel

//             if let Some(name) = input.name {
//                 active_item.name = Set(name);
//             }
//             if let Some(price) = input.price {
//                 active_item.price = Set(price);
//             }

//             // Execute the update
//             match active_item.update(&db).await {
//                 Ok(updated_item) => (StatusCode::OK, Json(updated_item)).into_response(),
//                 Err(_) => {
//                     (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update item").into_response()
//                 }
//             }
//         }
//         Ok(None) => (StatusCode::NOT_FOUND, "Item not found").into_response(),
//         Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
//     }
// }
