// main.rs
mod entities;
mod handler;
mod route;
mod db;
use db::connect;
//Decalares the `handlers` module from handlers.rs
// mod migrator;
use futures::executor::block_on;
// use sea_orm::{ConnectionTrait, Database, DbBackend, DbErr, Statement};
use std::net::SocketAddr;
// use sea_orm_migration::prelude::{MigratorTrait, MigrationTrait}; //Migration API
use sea_orm::DbErr;

async fn run() -> Result<(), DbErr> {
    // // Connect to the default postgres database
    // let _db = Database::connect("postgres://postgres:1002@localhost:5432/postgres").await?;
    // // Now connect to the newly created product_item_db
    // let url = format!("{}/{}", DATABASE_URL, DB_NAME);
    // let db = Database::connect(&url).await?;
    // Load environment variables from a `.env` file
    // Connect to the database
    let db = connect().await?;

    // Create the router
    let app = route::create_router(db);

    // Run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
    println!("DONE!");
}
