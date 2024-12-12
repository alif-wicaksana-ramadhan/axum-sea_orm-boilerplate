pub mod user_route;

use axum::Router;
use crate::interface::routes::user_route::create_user_router;

pub fn create_router() -> Router {
    let user_router = create_user_router();
    let router = Router::new()
        .nest("/user", user_router);
    router
}