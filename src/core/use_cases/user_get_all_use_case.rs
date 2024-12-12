use crate::infrastructure::repositories::user_repository::{UserRepository, UserRepositoryTrait};
use crate::infrastructure::entities::users;
pub struct GetAllUserUseCase<'a> {
    pub user_repo: &'a UserRepository,
}

impl <'a> GetAllUserUseCase<'a> {
    pub fn new(user_repo: &'a UserRepository) -> Self {
        Self { user_repo }
    }

    pub async fn execute(&self) -> Result<Vec<users::Model>, String> {
        let users: Vec<users::Model> = self.user_repo.find_all().await?;
        Ok(users)
    }
}