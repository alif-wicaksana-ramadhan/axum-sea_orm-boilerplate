use crate::infrastructure::entities::sea_orm_active_enums::UserRole;
use crate::infrastructure::entities::users;
use crate::infrastructure::repositories::user_repository::{UserRepository, UserRepositoryTrait};

pub struct RegisterUserUseCase<'a> {
    pub user_repo: &'a UserRepository,
}

impl<'a> RegisterUserUseCase<'a> {
    pub fn new(user_repo: &'a UserRepository) -> Self {
        RegisterUserUseCase { user_repo }
    }

    pub async fn execute(&self, username: String, email: String, role: UserRole, password: String) -> Result<users::Model, String> {
        if !email.contains('@') {
            return Err("Invalid email".to_string());
        }
        // let user_role = match role.as_str() {
        //     "admin" => UserRole::Admin,
        //     _ => UserRole::User,
        // };
        let user = users::Model {
            id: Default::default(),
            username,
            email,
            password,
            role//: user_role,
        };

        match self.user_repo.save(&user).await {
            Ok(_) => Ok(user),
            Err(e) => Err(e.to_string()),
        }
    }
}
