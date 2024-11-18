use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct DocumentationRequest {
    function_name: String,
}

#[derive(Serialize)]
struct DocumentationResponse {
    description: String,
    example: String,
}