use dotenvy::dotenv;
use crate::interface::routes::create_router;

mod interface;
mod application;
mod core;
mod infrastructure;

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load the environment variables

    let app = create_router();
    let addr: String = "0.0.0.0:8000".parse().unwrap();

    println!("Server running on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();
    axum::serve(listener, app)
        .await
        .unwrap();
}
