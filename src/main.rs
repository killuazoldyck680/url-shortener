
use axum::http::{StatusCode, Uri};
use axum::response::{Html, Redirect, IntoResponse, Response};
use axum::routing::post;
use axum::{Json, Router, routing::get};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
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

async fn home_page() -> Result<Html<String>, StatusCode> {
    match std::fs::read_to_string("templates/index.html") {
        Ok(html_content) => Ok(Html(html_content)),
        Err(_) => {
            println!("❌ Error: Could not find templates/index.html file!");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn shorten_url(Json(payload): Json<ShortenRequest>) -> Json<ShortenResponse> {
    let charset: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();

    let short_code: String = (0..6).map(|_| {
        let idx = rng.gen_range(0..charset.len());
        charset[idx] as char
    })
    .collect();

    let file_content = fs::read_to_string("links.json").unwrap_or_else(|_| "{}".to_string());
    let mut links: HashMap<String, String> = serde_json::from_str(&file_content).unwrap_or_default();

    links.insert(short_code.clone(), payload.long_url.clone());

    if let Ok(json_string) = serde_json::to_string_pretty(&links) {
        let _ = fs::write("links.json", json_string);
        println!("💾 Saved to file: {} -> {}", short_code, payload.long_url)
    }
    
    let dynamic_short_url = format!("http://127.0.0.1:5000/{}", short_code);
    Json(ShortenResponse { short_url: dynamic_short_url })
}

async fn redirect_to_url(uri: Uri) -> Response {
    // Manually grab the code straight out of the raw web address string
    let raw_path = uri.path().trim_start_matches('/');
    let search_code = raw_path.to_lowercase();

    
    if search_code == "favicon.ico" || search_code.is_empty() {
        return StatusCode::NOT_FOUND.into_response();
    }
    let file_content = fs::read_to_string("links.json").unwrap_or_else(|_| "{}".to_string());
    let links: HashMap<String,String> = serde_json::from_str(&file_content).unwrap_or_default();
    
    match links.get(&search_code) {
        Some(original_url) => {
            println!("🎯 Found mapping! Redirecting to: {}", original_url);
            Redirect::to(original_url).into_response()
        }
        None => {
            println!("⚠️ Code [{}] not found in links.json.", search_code);
            StatusCode::NOT_FOUND.into_response()
        }
    }
}