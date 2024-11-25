use sea_orm::{Database, DatabaseConnection};
use dotenvy::dotenv;
use std::env;

pub async fn establish_connection() -> DatabaseConnection{
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Database::connect(database_url).await.expect("Database connection failed")
}