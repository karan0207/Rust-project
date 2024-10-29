use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
}

// Shared state to hold a list of users (using Mutex to allow safe access across threads)
struct AppState {
    users: Mutex<Vec<User>>,
}

// Health check endpoint
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Server is running!")
}

// Get all users
async fn get_users(state: web::Data<AppState>) -> impl Responder {
    let users = state.users.lock().unwrap();
    HttpResponse::Ok().json(users.clone())
}

// Add a new user
async fn add_user(new_user: web::Json<User>, state: web::Data<AppState>) -> impl Responder {
    let mut users = state.users.lock().unwrap();
    users.push(new_user.into_inner());
    HttpResponse::Ok().body("User added!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    env_logger::init();

    // Create initial state with some users
    let app_state = web::Data::new(AppState {
        users: Mutex::new(vec![
            User { id: 1, name: "Karan".to_string() },
            User { id: 2, name: "Alice".to_string() },
        ]),
    });

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())  // Share app state across workers
            .route("/", web::get().to(health_check))
            .route("/users", web::get().to(get_users))
            .route("/users", web::post().to(add_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
