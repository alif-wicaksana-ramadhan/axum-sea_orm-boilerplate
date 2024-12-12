use axum::{Router, routing::post};
use axum::routing::get;
use crate::interface::handlers::user_handler::{register_user, get_all_users};

pub fn create_user_router() -> Router {
    Router::new()
        .route("/register", post(register_user))
        .route("/", get(get_all_users))
}
