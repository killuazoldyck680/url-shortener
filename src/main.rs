use axum::extract::State;
use axum::http::{StatusCode, Uri};
use axum::response::Redirect;
use axum::routing::post;
use axum::{Json, Router, routing::get};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

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

    // We use .fallback() to catch all incoming GET requests manually.
    // This completely removes Axum's internal macro matching bugs!
    let app = Router::new()
        .route("/", get(home_page))
        .route("/shorten", post(shorten_url))
        .fallback(get(redirect_to_url))
        .with_state(shared_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 5000));
    println!("🚀 Server running smoothly at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn home_page() -> &'static str {
    "Welcome to my solo URL Shortener API!"
}

async fn shorten_url(State(state): State<AppState>, Json(payload): Json<ShortenRequest>) -> Json<ShortenResponse> {
    let charset: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();

    let short_code: String = (0..6).map(|_| {
        let idx = rng.gen_range(0..charset.len());
        charset[idx] as char
    })
    .collect();

    let mut map = state.lock().unwrap();
    map.insert(short_code.clone(), payload.long_url);

    // Flat, clean short link format
    let dynamic_short_url = format!("http://127.0.0.1:5000/{}", short_code);
    Json(ShortenResponse { short_url: dynamic_short_url })
}

async fn redirect_to_url(State(state): State<AppState>, uri: Uri) -> Result<Redirect, StatusCode> {
    // Manually grab the code straight out of the raw web address string
    let raw_path = uri.path().trim_start_matches('/');
    let search_code = raw_path.to_lowercase();

    // Ignore browser system icon queries so they don't break our data flow
    if search_code == "favicon.ico" || search_code.is_empty() {
        return Err(StatusCode::NOT_FOUND);
    }

    println!("🔍 Processing redirect for code: [{}]", search_code);

    let map = state.lock().unwrap();
    println!("📋 Current DB entries: {:?}", map.keys());

    match map.get(&search_code) {
        Some(original_url) => {
            println!("🎯 Success! Redirecting to: {}", original_url);
            // Passed correctly as a borrowed reference string slice (&str)
            Ok(Redirect::to(original_url))
        }
        None => {
            println!("⚠️ Warning: Code [{}] not found in database.", search_code);
            Err(StatusCode::NOT_FOUND)
        }
    }
}