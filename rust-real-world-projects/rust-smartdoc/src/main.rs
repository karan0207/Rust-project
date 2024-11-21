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

async fn get_documentation(info: web::Json<DocumentationRequest>) -> impl Responder {
    let function_name = &info.function_name;

    // Simulated response (you can use rustdoc data here)
    let response = DocumentationResponse {
        description: format!("Detailed explanation of function `{}`", function_name),
        example: format!(
            "Example usage of `{}`:\n\nfn main() {{\n    println!(\"Hello, {}!\");\n}}",
            function_name, function_name
        ),
    };

    HttpResponse::Ok().json(response)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/documentation", web::post().to(get_documentation))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}