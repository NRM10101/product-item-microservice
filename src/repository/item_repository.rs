use crate::entities::item::{
    self, ActiveModel as ItemActiveModel, CreateItemModel, Entity as ItemEntity, UpdateItemModel,
};
use sea_orm::{entity::*, DatabaseConnection, Set};
use tracing::{error, info, warn};

pub struct ItemRepository {
    db: DatabaseConnection,
}
impl ItemRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn insert_item(&self, data: CreateItemModel) -> Result<(), String> {
        info!("Repository: Inserting new item with name '{}'", data.name);
        let item_model = ItemActiveModel {
            product_id: Set(data.product_id),
            name: Set(data.name.clone()),
            price: Set(data.price),
            ..Default::default()
        };
        ItemEntity::insert(item_model)
            .exec(&self.db)
            .await
            .map_err(|err| {
                error!("Repository: Failed to insert item '{}': {}", data.name, err);
                err.to_string()
            })?;
        info!("Repository: Item '{}' successfully inserted", data.name);
        Ok(())
    }

    // Update item with detailed tracing
    pub async fn update_item(&self, data: UpdateItemModel) -> Result<(), String> {
        info!("Repository: Attempting to update item ID {}", data.id);

        if let Some(item) = ItemEntity::find_by_id(data.id)
            .one(&self.db)
            .await
            .map_err(|err| {
                error!(
                    "Repository: Failed to find item ID {} for update: {}",
                    data.id, err
                );
                err.to_string()
            })?
        {
            let mut am = item.into_active_model();

            if let Some(product_id) = data.product_id {
                am.product_id = Set(product_id);
            }
            if let Some(name) = data.name {
                am.name = Set(name);
            }
            if let Some(price) = data.price {
                am.price = Set(price);
            }

            am.update(&self.db).await.map_err(|err| {
                error!("Repository: Failed to update item ID {}: {}", data.id, err);
                err.to_string()
            })?;

            info!("Repository: Item ID {} successfully updated", data.id);
            Ok(())
        } else {
            error!("Repository: Item ID {} not found", data.id);
            Err("Item not found".to_string())
        }
    }

    pub async fn get_all_items(&self) -> Result<Vec<item::Model>, String> {
        info!("Repository: Querying all items");
        item::Entity::find().all(&self.db).await.map_err(|err| {
            error!("Repository: Failed to fetch all items - {}", err);
            err.to_string()
        })
    }

    pub async fn get_item_by_id(&self, id: i32) -> Result<Option<item::Model>, String> {
        info!("Repository: Querying item by ID - {}", id);
        return item::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|err| {
                error!("Repository: Failed to fetch item by ID {}: {}", id, err);
                err.to_string()
            });
    }

    pub async fn delete_item_by_id(&self, id: i32) -> Result<(), String> {
        info!("Repository: Attempting to delete item by ID - {}", id);
        match item::Entity::find_by_id(id).one(&self.db).await {
            Ok(Some(item)) => item.delete(&self.db).await.map(|_| ()).map_err(|err| {
                error!("Repository: Failed to delete item by ID {}: {}", id, err);
                err.to_string()
            }),
            Ok(None) => {
                warn!("Repository: No item found to delete by ID - {}", id);
                Err("Item not found".to_string())
            }
            Err(err) => {
                error!(
                    "Repository: Database error during deletion of item by ID {}: {}",
                    id, err
                );
                Err(err.to_string())
            }
        }
    }
}
