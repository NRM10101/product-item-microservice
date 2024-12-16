use crate::entities::item::{self, CreateItemModel, UpdateItemModel};
use sea_orm::DatabaseConnection;
use tracing::{error, info, warn};
use crate::repository::item_repository::ItemRepository;

// pub async fn create_item(db: DatabaseConnection, data: CreateItemModel) -> Result<(), String> {
//     let repo = ItemRepository::new(db);
//     repo.insert_item(data).await
// }

// pub async fn update_item(db: DatabaseConnection, data: UpdateItemModel) -> Result<(), String> {
//     let repo = ItemRepository::new(db);
//     repo.update_item(data).await
// }
pub async fn create_item(db: DatabaseConnection, data: CreateItemModel) -> Result<(), String> {
    let repo = ItemRepository::new(db);
    info!("Service: Starting to create item with name '{}'", data.name);
    let item_name = data.name.clone();
    match repo.insert_item(data).await {
        Ok(_) => {
            info!("Service: Successfully created item '{}'", item_name);
            Ok(())
        },
        Err(e) => {
            error!("Service: Failed to create item '{}': {}", item_name, e);
            Err(e)
        }
    }
}

// Service function to update an item with detailed tracing
pub async fn update_item(db: DatabaseConnection, data: UpdateItemModel) -> Result<(), String> {
    let repo = ItemRepository::new(db);
    info!("Service: Starting to update item ID {}", data.id);
    let item_id = data.id;
    match repo.update_item(data).await {
        Ok(_) => {
            info!("Service: Successfully updated item ID {}", item_id);
            Ok(())
        },
        Err(e) => {
            error!("Service: Failed to update item ID {}: {}", item_id, e);
            Err(e)
        }
    }
}

pub async fn get_all_items(db: &DatabaseConnection) -> Result<Vec<item::Model>, String> {
    info!("Service: Requesting all items from repository");
    let result = ItemRepository::new(db.clone()).get_all_items().await;
    if result.is_ok() {
        info!("Service: Received all items successfully");
    } else {
        error!("Service: Failed to retrieve all items");
    }
    result
}

pub async fn get_item_by_id(
    db: &DatabaseConnection,
    id: i32,
) -> Result<Option<item::Model>, String> {
    info!("Service: Requesting item by ID from repository - ID {}", id);
    let result = ItemRepository::new(db.clone()).get_item_by_id(id).await;
    match &result {
        Ok(Some(_)) => info!("Service: Item found - ID {}", id),
        Ok(None) => warn!("Service: No item found - ID {}", id),
        Err(e) => error!("Service: Error retrieving item by ID {}: {}", id, e),
    }
    result
}

pub async fn delete_item_by_id(db: &DatabaseConnection, id: i32) -> Result<(), String> {
    info!("Service: Requesting deletion of item by ID - ID {}", id);
    let result = ItemRepository::new(db.clone()).delete_item_by_id(id).await;
    if result.is_ok() {
        info!("Service: Item deleted successfully - ID {}", id);
    } else {
        error!("Service: Failed to delete item by ID {}", id);
    }
    result
}
