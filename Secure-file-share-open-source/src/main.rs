use actix_files::Files;
use actix_web::{web, App, HttpServer};
use rsa::{RsaPrivateKey, RsaPublicKey};
use std::sync::Arc;
use std::fs;

mod routes;
mod encryption;
mod file_handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let private_key = fs::read_to_string("keys/private_key.pem").expect("Failed to read private key");
    let public_key = fs::read_to_string("keys/public_key.pem").expect("Failed to read public key");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(RsaPrivateKey::from_pem_str(&private_key).unwrap()))
            .app_data(web::Data::new(RsaPublicKey::from_pem_str(&public_key).unwrap()))
            .service(web::scope("/api").configure(routes::configure))
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
