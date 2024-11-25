mod db;
mod api;
mod services;
mod repositories;
mod extensions;

use std::collections::HashMap;
use std::sync::Arc;
use axum::{Extension, Json, Router};
use axum::extract::Query;
use axum::http::StatusCode;
use axum::routing::{get, post};
use serde::{Deserialize, Serialize};
use crate::extensions::auth::AuthState;

#[tokio::main]
async fn main() {
    let db = Arc::new(db::establish_connection().await);
    let auth = Arc::new(AuthState::new("valid_secret".to_string()));

    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
        .layer(Extension(db))
        .layer(Extension(auth));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn get_user(Query(params): Query<HashMap<String, String>>) {

}

async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}