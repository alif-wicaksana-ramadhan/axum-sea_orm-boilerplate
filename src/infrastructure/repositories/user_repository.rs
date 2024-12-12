use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};
use sea_orm::prelude::async_trait::async_trait;
use crate::infrastructure::entities::users;
use crate::infrastructure::database::establish_connection;

#[async_trait]
pub trait UserRepositoryTrait {
    async fn save(&self, user: &users::Model) -> Result<users::Model, String>;
    async fn find_all(&self) -> Result<Vec<users::Model>, String>;
    async fn find_by_id(&self, id: i32) -> Result<Option<users::Model>, String>;
}

pub struct UserRepository;

#[async_trait]
impl UserRepositoryTrait for UserRepository {
    async fn save(&self, user: &users::Model) -> Result<users::Model, String> {
        let conn = establish_connection()
            .await;
        let new_user = users::ActiveModel {
            id: ActiveValue::NotSet, // Assuming `id` is auto-generated
            username: ActiveValue::Set(user.username.clone()),
            email: ActiveValue::Set(user.email.clone()),
            password: ActiveValue::Set(user.password.clone()),
            role: ActiveValue::Set(user.role.clone()),
            ..Default::default()
        };
        let inserted_user = new_user.insert(&conn)
            .await
            .map_err(|e| e.to_string())?;
        Ok(inserted_user)
    }

    async fn find_all(&self) -> Result<Vec<users::Model>, String> {
        let conn = establish_connection()
            .await;
        let users: Vec<users::Model> = users::Entity::find()
            .all(&conn)
            .await
            .map_err(|e| e.to_string())?;
        Ok(users)
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<users::Model>, String> {
        let conn = establish_connection()
            .await;
        let user: Option<users::Model> = users::Entity::find_by_id(id)
            .one(&conn)
            .await
            .map_err(|e| e.to_string())?;
        Ok(user)
    }
}

