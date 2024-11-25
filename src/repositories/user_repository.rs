use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct UserRepository {
    pub db: DatabaseConnection
}

impl UserRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    // pub async fn find_all(&self) -> Resu
}