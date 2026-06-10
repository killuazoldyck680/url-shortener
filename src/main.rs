use axum::extract::State;
use axum::routing::post;
use axum::{Router, routing::get, Json};
use std::net::SocketAddr;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use rand::{distributions::Alphanumeric, Rng};

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
    .route("/shorten", post(shorten_url))
    .with_state(shared_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running up at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn home_page() -> &'static str {
    "Welcome to my solo URL Shortener API!"
}

async fn shorten_url(State(state): State<AppState>,Json(payload): Json<ShortenRequest>) -> Json<ShortenResponse> {
    let short_code: String = rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(6)
    .map(char::from)
    .collect();

    let mut map = state.lock().unwrap();
    map.insert(short_code.clone(), payload.long_url);

    let dynamic_short_url = format!("http://127.0.0.1:3000/{}", short_code);

    Json(ShortenResponse { short_url: dynamic_short_url })
}
