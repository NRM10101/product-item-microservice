use sea_orm_migration::prelude::*;

// #[derive(DeriveMigrationName)]
pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20241205_170012_create_product_table" // Make sure this matches with the file name
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
                    .table(Product::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Product::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Product::Name).string().not_null())
                    .col(ColumnDef::new(Product::Description).string().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();
        manager
            .drop_table(Table::drop().table(Product::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Product {
    Table,
    Id,
    Name,
    Description,
}
