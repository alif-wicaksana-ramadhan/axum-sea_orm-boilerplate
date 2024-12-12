use axum::{Json, response::IntoResponse, Error};
use crate::core::use_cases::user_get_all_use_case::GetAllUserUseCase;
use crate::core::use_cases::user_register_use_case::RegisterUserUseCase;
use crate::infrastructure::repositories::user_repository::UserRepository;
use crate::core::models::user_model;
use crate::infrastructure::entities::sea_orm_active_enums::UserRole;

fn role_from_string(role_str: String) -> Result<UserRole, Error> {
    match role_str.as_str() {
        "admin" => Ok(UserRole::Admin),
        "user" => Ok(UserRole::User),
        _ => Err(axum::Error::from(
            std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid Role"),
        ))
    }
}

pub async fn register_user(Json(payload): Json<user_model::Model>) -> impl IntoResponse {
    let user_repo = UserRepository;
    let register_user_use_case = RegisterUserUseCase::new(&user_repo);
    let user_role = match role_from_string(payload.role) {
        Ok(role) => {
            match register_user_use_case.execute(
                payload.username,
                payload.email,
                role,
                payload.password
            ).await {
                Ok(user) => Json(user).into_response(),
                Err(err) => (axum::http::StatusCode::BAD_REQUEST, err).into_response(),
            }
        },
        Err(_) => (axum::http::StatusCode::BAD_REQUEST, "Invalid role").into_response()
    };
    user_role
}

pub async fn get_all_users() -> impl IntoResponse {
    let user_repo = UserRepository;
    let get_all_user_use_case = GetAllUserUseCase::new(&user_repo);
    match get_all_user_use_case.execute().await {
        Ok(users) => Json(users).into_response(),
        Err(err) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, err).into_response(),
    }
}
