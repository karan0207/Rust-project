use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use rusqlite::{params, Connection};
use std::sync::Mutex;

mod db;

// Request structure for shortening URL
#[derive(Serialize, Deserialize)]
struct UrlRequest {
    url: String,
}

// Response structure for returning shortened URL
#[derive(Serialize, Deserialize)]
struct UrlResponse {
    short_url: String,
}

async fn shorten_url(
    data: web::Data<Mutex<Connection>>,
    req: web::Json<UrlRequest>,
) -> impl Responder {
    let conn = data.lock().unwrap();
    let short_code = db::insert_url(&conn, &req.url).unwrap();
    let response = UrlResponse {
        short_url: format!("http://localhost:8080/{}", short_code),
    };
    HttpResponse::Ok().json(response)
}

async fn redirect_url(
    data: web::Data<Mutex<Connection>>,
    web::Path(short_code): web::Path<String>,
) -> impl Responder {
    let conn = data.lock().unwrap();
    match db::get_url(&conn, &short_code) {
        Some(url) => HttpResponse::Found().header("Location", url).finish(),
        None => HttpResponse::NotFound().body("URL not found"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize SQLite connection
    let conn = db::init_db().expect("Failed to initialize database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Mutex::new(conn.clone())))
            .route("/shorten", web::post().to(shorten_url)) // POST /shorten
            .route("/{short_code}", web::get().to(redirect_url)) // GET /<short_code>
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
