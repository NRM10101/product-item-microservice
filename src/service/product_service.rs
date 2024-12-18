use crate::{
    entities::product::{self, CreateProductModel, UpdateProductModel},
    repository::product_repository::ProductRepository,
};
use sea_orm::DatabaseConnection;
use tracing::{error, info};

pub async fn create_product(
    db: DatabaseConnection,
    product_data: CreateProductModel,
) -> Result<(), String> {
    info!("Service: Starting product creation");
    let repo = ProductRepository::new(db);
    match repo.insert_product(product_data).await {
        Ok(_) => {
            info!("Service: Product created successfully");
            Ok(())
        }
        Err(e) => {
            error!("Service: Error creating product: {}", e);
            Err(e.to_string())
        }
    }
}

pub async fn update_product(
    db: DatabaseConnection,
    product_data: UpdateProductModel,
) -> Result<(), String> {
    info!("Service: Starting product update");
    let repo = ProductRepository::new(db);
    match repo.update_product(product_data).await {
        Ok(_) => {
            info!("Service: Product updated successfully");
            Ok(())
        }
        Err(e) => {
            error!("Service: Error updating product: {}", e);
            Err(e.to_string())
        }
    }
}

pub async fn get_all_products(db: &DatabaseConnection) -> Result<Vec<product::Model>, String> {
    info!("Service: Fetching all products.");
    ProductRepository::new(db.clone()).get_all_products().await
}

pub async fn get_product_by_id(
    db: &DatabaseConnection,
    id: i32,
) -> Result<Option<product::Model>, String> {
    info!("Service: Fetching product by ID - {}", id);
    ProductRepository::new(db.clone())
        .get_product_by_id(id)
        .await
}

pub async fn delete_product_by_id(db: &DatabaseConnection, id: i32) -> Result<(), String> {
    info!("Service: Deleting product by ID - {}", id);
    ProductRepository::new(db.clone())
        .delete_product_by_id(id)
        .await
}
