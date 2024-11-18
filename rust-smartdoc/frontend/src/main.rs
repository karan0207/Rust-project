use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use serde::{Deserialize, Serialize};
use gloo_net::http::Request;


#[derive(Serialize)]
struct DocumentationRequest {
    function_name: String,
}

#[derive(Deserialize, Debug)]
struct DocumentationResponse {
    description: String,
    example: String,
}