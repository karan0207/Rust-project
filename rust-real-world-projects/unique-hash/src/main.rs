use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use rand::Rng;
use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};
use hex::encode;

#[get("/generate_hash")]
async fn generate_hash() -> impl Responder {
    // Get the current timestamp in seconds since the epoch
    let start = SystemTime::now();
    let timestamp = start.duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();

    // Generate a random number for uniqueness
    let random_number: u64 = rand::thread_rng().gen();

    // Create a unique input by combining the timestamp and random number
    let unique_input = format!("{}-{}", timestamp, random_number);

    // Generate a SHA-256 hash of the unique input
    let mut hasher = Sha256::new();
    hasher.update(unique_input);
    let hash_result = hasher.finalize();

    // Convert the hash to a hexadecimal string
    let hash_string = encode(hash_result);

    HttpResponse::Ok().body(hash_string)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Start the Actix server
    HttpServer::new(|| {
        App::new()
            .service(generate_hash) // Register the hash generation endpoint
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
