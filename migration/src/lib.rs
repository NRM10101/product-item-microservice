pub use sea_orm_migration::prelude::*;

mod m20241205_170012_create_product_table;
mod m20241205_170022_create_item_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241205_170012_create_product_table::Migration),
            Box::new(m20241205_170022_create_item_table::Migration),
        ]
    }
}
