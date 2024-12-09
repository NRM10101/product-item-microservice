use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension,body::Full,
};
use crate::entities::product;
use sea_orm::{DatabaseConnection, EntityTrait};
use serde_json::to_vec; // Serialize json data to Vec<u8>

pub async fn get_all_products(
    Extension(db): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    match query_all_products(&db).await {
        Ok(products) => {
            let serialized = to_vec(&products).unwrap(); // Serialize Vec<product::Model> to Vec<u8>
            Response::builder()
                .status(StatusCode::OK)
                .body(Full::from(serialized)) // Use Full with serialized data
                .unwrap()
        },
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
