use crate::entities::product::{
    self, ActiveModel as ProductActiveModel, CreateProductModel, Entity as ProductEntity,
    UpdateProductModel,
};
use sea_orm::{entity::*, DatabaseConnection, Set, TransactionTrait};
use tracing::{error, info, warn};

pub struct ProductRepository {
    db: DatabaseConnection,
}

impl ProductRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn insert_product(
        &self,
        product_data: CreateProductModel,
    ) -> Result<(), sea_orm::DbErr> {
        info!("Repository: Inserting new product into the database");
        let product_model = ProductActiveModel {
            name: Set(product_data.name),
            description: Set(Some(product_data.description)),
            ..Default::default()
        };
        ProductEntity::insert(product_model)
            .exec(&self.db)
            .await
            .map(|_| ())
            .map_err(|e| {
                error!("Repository: Failed to insert product: {:?}", e);
                e
            })
    }

    pub async fn update_product(
        &self,
        product_data: UpdateProductModel,
    ) -> Result<(), sea_orm::DbErr> {
        info!("Repository: Updating product in the database");
        let mut transaction = self.db.begin().await?;

        if let Some(product) = ProductEntity::find_by_id(product_data.id)
            .one(&mut transaction)
            .await?
        {
            let mut am = product.into_active_model();
            if let Some(name) = product_data.name {
                am.name = Set(name);
            }
            if let Some(description) = product_data.description {
                am.description = Set(Some(description));
            }
            am.update(&mut transaction).await?;
            transaction.commit().await.map_err(|e| {
                error!("Repository: Failed to commit transaction: {:?}", e);
                e
            })
        } else {
            transaction.rollback().await?;
            Err(sea_orm::DbErr::RecordNotFound(
                "Product not found".to_string(),
            ))
        }
    }
    pub async fn get_all_products(&self) -> Result<Vec<product::Model>, String> {
        info!("Repository: Querying all products.");
        product::Entity::find().all(&self.db).await.map_err(|err| {
            error!("Repository: Failed to fetch all products - {}", err);
            err.to_string()
        })
    }

    pub async fn get_product_by_id(&self, id: i32) -> Result<Option<product::Model>, String> {
        info!("Repository: Querying product by ID - {}", id);
        product::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|err| {
                error!("Repository: Failed to fetch product by ID {}: {}", id, err);
                err.to_string()
            })
    }

    pub async fn delete_product_by_id(&self, id: i32) -> Result<(), String> {
        info!("Repository: Deleting product by ID - {}", id);
        match product::Entity::find_by_id(id).one(&self.db).await {
            Ok(Some(product)) => product.delete(&self.db).await.map(|_| ()).map_err(|err| {
                error!("Repository: Failed to delete product by ID {}: {}", id, err);
                err.to_string()
            }),
            Ok(None) => {
                warn!("Repository: No product found to delete by ID - {}", id);
                Err("Product not found".to_string())
            }
            Err(err) => {
                error!(
                    "Repository: Database error during deletion of product by ID {}: {}",
                    id, err
                );
                Err(err.to_string())
            }
        }
    }
}
