use actix_web::{post, get, web, HttpResponse};
use crate::file_handler::{save_file, load_file};
use crate::encryption::{encrypt, decrypt};
use rsa::{RsaPrivateKey, RsaPublicKey};
use std::sync::Arc;

#[post("/upload")]
async fn upload_file(file: web::Bytes, public_key: web::Data<RsaPublicKey>) -> HttpResponse {
    let encrypted_file = encrypt(&public_key, &file);
    save_file("encrypted_file.bin", &encrypted_file);
    HttpResponse::Ok().body("File uploaded and encrypted successfully")
}

#[get("/download")]
async fn download_file(private_key: web::Data<RsaPrivateKey>) -> HttpResponse {
    let encrypted_file = load_file("encrypted_file.bin");
    let decrypted_file = decrypt(&private_key, &encrypted_file);
    HttpResponse::Ok().body(decrypted_file)
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(upload_file);
    cfg.service(download_file);
}
