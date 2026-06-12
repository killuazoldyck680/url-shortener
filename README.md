🚀 Secure & Lightweight URL Shortener
A high-performance, asynchronous web application built with Rust using the Axum framework. This service provides a friction-free, local filesystem-backed data engine to reliably shorten web links while employing rigid input safety filtering to guard against redirection loops and injection vulnerabilities.

✨ Features
High Performance Architecture: Powered by Axum v0.7 and the Tokio runtime engine for top-tier asynchronous concurrency handling.

Low-Friction Persistence Layer: Fast local database layout utilizing a dynamic JSON file mapping structure (links.json) requiring no database setups or complex macro states.

Rigid Server-Side URL Validation: Powered by the robust url parser crate to reject structured malformations natively.

Infinite Redirection Loop Protection: Active security layer automatically filters out loops targeting localhost, 127.0.0.1, or 0.0.0.0.

Protocol Whitelisting: Strictly enforces incoming parameters to use valid http:// or https:// schemas.

Fluid UX Design: Modern dark-mode theme interface with one-click Copy-to-Clipboard asynchronous JavaScript macro support.

🛠️ Tech Stack & Dependencies
Language: Rust (Edition 2021)

Web Framework: axum (Asynchronous HTTP routing)

Async Runtime: tokio (Multi-threaded worker pool)

Serialization/Deserialization: serde & serde_json

Data Verification: url (Strict standard compliance checks)

Randomizer Engine: rand (Thread-safe cryptographic scaling vectors)

📂 Project Structure
Plaintext
url-shortener/
├── Cargo.toml # Rust dependency manifests
├── links.json # File system database (created on runtime)
├── src/
│ └── main.rs # Axum server routes, security engines, & handlers
└── templates/
└── index.html # Responsive frontend interface & script pipelines
🚀 Quick Start Guide

1. Prerequisites
   Ensure you have the latest stable version of the Rust toolchain installed. If not, set it up via rustup.rs:

PowerShell
rustup update stable 2. Dependency Setup
Verify that your local Cargo.toml file includes the exact configured dependencies below:

Ini, TOML
[dependencies]
axum = "0.7.9"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
rand = "0.8"
url = "2.5" 3. Compilation & Boot Execution
Navigate to your project root directory and spin up the server environment directly:

PowerShell

# Purge cached artifacts and run clean binary builds

cargo clean
cargo run
Once compilation finishes, your console logs will verify active execution state tracking:


🚀 Launching secure, validated JSON-file shortener...
🚀 Server running smoothly at http://127.0.0.1:5000
Open up your web browser and navigate directly to http://127.0.0.1:5000 to begin using your app!

🛡️ API Endpoints Reference

1. Serve Graphical Frontend
   Method: GET

Path: /

Response: 200 OK | text/html markup interface.

2. Generate Minified Reference Mapping
   Method: POST

Path: /shorten

Headers: Content-Type: application/json

Request Payload Example:

JSON
{
"long_url": "https://doc.rust-lang.org/book/"
}
Success Response (201 Created):

JSON
{
"short_url": "http://127.0.0.1:5000/x4z2p9"
}
Error Response (400 Bad Request): Returned if the payload fails verification steps, such as passing loop inputs or missing schemas.


Error: Cannot shorten URLs pointing back to this server. 3. Dynamic Forwarding Redirection
Method: GET

Path: /:short_code (e.g., /x4z2p9)

Behavior: Redirects seamlessly with an HTTP 303 See Other location update mapping back to the destination. If the identifier is missing inside links.json, it raises a native 404 Not Found response.
