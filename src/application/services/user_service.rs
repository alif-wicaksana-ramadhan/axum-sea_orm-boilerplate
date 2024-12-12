use crate::infrastructure::entities::sea_orm_active_enums::UserRole;
use crate::infrastructure::entities::users;
use crate::infrastructure::repositories::user_repository::{UserRepository, UserRepositoryTrait};

pub struct UserService;

impl UserService {
    pub async fn register_user(email: String, username: String, password: String, role: String) -> Result<users::Model, String> {
        if !email.contains('@') {
            return Err("Invalid email".to_string());
        }
        let user_repo = UserRepository;
        let user_role = match role.as_str() {
            "admin" => UserRole::Admin,
            _ => UserRole::User,
        };
        let user = users::Model {
            id: Default::default(),
            username,
            email,
            password,
            role: user_role,
        };

        match user_repo.save(&user).await {
            Ok(_) => Ok(user),
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn find_all() -> Result<Vec<users::Model>, String> {
        let user_repo = UserRepository;
        let users: Vec<users::Model> = user_repo.find_all().await?;
        Ok(users)
    }
}
