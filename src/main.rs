use axum::{Router, routing::get};
use std::net::SocketAddr;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};

type AppState = Arc<Mutex<HashMap<String, String>>>;

#[derive(Deserialize)]
struct ShortenRequest {
    long_url: String,
}

#[derive(Serialize)]
struct ShortenResponse {
    short_url: String,
}



#[tokio::main]
async fn main() {

    let shared_state: AppState = Arc::new(Mutex::new(HashMap::new()));


    let app = Router::new().route("/", get(home_page))
    .with_state(shared_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running up at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn home_page() {
    println!("Welcome to my solo URL Shortener API!")
}
