use axum::{Router, routing::get};
use std::net::SocketAddr;





#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(home_page));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running up at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn home_page() {
    println!("Welcome to my solo URL Shortener API!")
}
