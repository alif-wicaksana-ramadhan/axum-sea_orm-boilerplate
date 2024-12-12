use sea_orm::{DatabaseConnection, DbErr};
use dotenvy::dotenv;
use std::env;

pub async fn establish_connection() -> DatabaseConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL not set in .env");

    sea_orm::Database::connect(&db_url)
        .await
        .expect("Error connecting to database")
}
