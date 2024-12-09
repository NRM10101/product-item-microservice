use dotenv::dotenv;
use sea_orm::{Database, DatabaseConnection, DbErr};
use std::env;

/// Function to establish a connection to the database
pub async fn connect() -> Result<DatabaseConnection, DbErr> {
    // Load environment variables from a `.env` file
    dotenv().ok();

    // Read environment variables
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_name = env::var("DB_NAME").expect("DB_NAME must be set");

    // Connect to the target database
    let full_url = format!("{}/{}", database_url, db_name);
    let db = Database::connect(&full_url).await?;
    Ok(db)
}
