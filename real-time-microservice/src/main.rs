use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use dotenv::dotenv;

mod websocket;
use websocket::ws_index;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load environment variables
    HttpServer::new(|| {
        App::new()
            .route("/ws/", web::get().to(ws_index)) // WebSocket route
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
