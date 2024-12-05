use sea_orm_migration::prelude::*;

use super::m20241205_170012_create_product_table::Product;

// #[derive(DeriveMigrationName)]
pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20241205_170022_create_item_table" // Make sure this matches with the file name
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();
        manager
            .create_table(
                Table::create()
                    .table(Item::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Item::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Item::ProductId).integer().not_null())
                    .col(ColumnDef::new(Item::Name).string().not_null())
                    .col(ColumnDef::new(Item::Price).decimal().not_null())
                    .foreign_key(ForeignKey::create()
                        .name("fk-product-id")
                        .from(Item::Table, Item::ProductId)
                        .to(Product::Table, Product::Id))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();
        manager
            .drop_table(Table::drop().table(Item::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Item {
    Table,
    Id,
    Name,
    Price,
    ProductId
}
