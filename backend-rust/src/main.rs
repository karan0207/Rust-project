use axum::{
    routing::{get, post},
    Router,
    Json,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use uuid::Uuid;

// Define a struct for your data
#[derive(Serialize, Deserialize, Clone, Debug)]
struct User {
    id: Uuid,
    name: String,
    email: String,
}

// An in-memory store for simplicity
use std::sync::{Arc, Mutex};
type Db = Arc<Mutex<Vec<User>>>;

#[tokio::main]
async fn main() {
    // Create the shared database
    let db = Arc::new(Mutex::new(vec![]));

    // Define routes
    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
        .route("/users/:id", get(get_user))
        .layer(axum::AddExtensionLayer::new(db));

    // Start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Server running on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Handlers

// Root handler
async fn root() -> &'static str {
    "Welcome to the Rust Backend API!"
}

// Handler to create a new user
async fn create_user(
    Json(payload): Json<CreateUserRequest>,
    db: axum::extract::Extension<Db>,
) -> Json<User> {
    let user = User {
        id: Uuid::new_v4(),
        name: payload.name,
        email: payload.email,
    };

    // Insert user into the in-memory database
    let mut db = db.lock().unwrap();
    db.push(user.clone());

    Json(user)
}

// Handler to get a user by ID
async fn get_user(
    axum::extract::Path(id): axum::extract::Path<Uuid>,
    db: axum::extract::Extension<Db>,
) -> Json<Option<User>> {
    let db = db.lock().unwrap();
    let user = db.iter().find(|user| user.id == id).cloned();
    Json(user)
}

// Request struct for creating a user
#[derive(Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
}
