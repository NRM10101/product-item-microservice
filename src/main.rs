// main.rs
mod entities;
mod handlers;
//Decalares the `handlers` module from handlers.rs
// mod migrator;
use futures::executor::block_on;
// use sea_orm::{ConnectionTrait, Database, DbBackend, DbErr, Statement};
use sea_orm::*;

use axum::{
    routing::{get, post},
    Extension, Router,
};
use entities::{prelude::*, *};
use handlers::*;
use sea_orm_migration::prelude::*; //Migration API

// Change this according to your database implementation,
// or supply it as an environment variable.
// the whole database URL string follows the following format:
// "protocol://username:password@host:port/database"
// We put the database name (that last bit) in a separate variable simply for convenience.
const DATABASE_URL: &str = "postgres://postgres:1002@localhost:5432";
const DB_NAME: &str = "product_item_db";

async fn run() -> Result<(), DbErr> {
    // Connect to the default postgres database
    let _db = Database::connect("postgres://postgres:1002@localhost:5432/postgres").await?;
    // Now connect to the newly created product_item_db
    let url = format!("{}/{}", DATABASE_URL, DB_NAME);
    let db = Database::connect(&url).await?;

    // Example: Insert a product
    let happy_product = product::ActiveModel {
        name: ActiveValue::Set("Happy Product".to_owned()),
        description: ActiveValue::Set(Some(
            "A delicious and freshly baked product that brings joy to your day.".to_owned(),
        )),
        ..Default::default()
    };

    let res = Product::insert(happy_product).exec(&db).await?;

    // Update the product
    let sad_product = product::ActiveModel {
        id: ActiveValue::Set(res.last_insert_id),
        name: ActiveValue::Set("Sad Bakery".to_owned()),
        description: ActiveValue::Set(Some(
            "A delicious and freshly baked product that brings joy to your day.".to_owned(),
        )),
    };
    sad_product.update(&db).await?;

    // // Create the router
    // let app = Router::new()
    //     .route("/products", get(get_all_products).post(add_product))
    //     .layer(Extension(db));

    // // Run the server
    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // println!("Listening on {}", addr);
    // axum::Server::bind(&addr)
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();
    Ok(())
}

// async fn query_all_products(db: &DatabaseConnection) -> Result<Vec<product::Model>, DbErr> {
//     Product::find().all(db).await
// }

#[tokio::main]
async fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
    println!("DONE!");
}
