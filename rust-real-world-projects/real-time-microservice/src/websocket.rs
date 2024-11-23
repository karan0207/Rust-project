use actix::prelude::*;
use actix_web::{HttpRequest, Responder};
use actix_web_actors::ws;
use crate::redis_pubsub::broadcast_message;

// Define a struct to handle WebSocket messages
pub struct WsChatSession;

// Implement the WebSocket actor for handling client connections
impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("New WebSocket connection established!");
    }
}

// Implement WebSocket message handler
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                println!("Received message: {}", text);
                let _ = broadcast_message(text.clone()); // Broadcast message to Redis
                ctx.text(format!("Echo: {}", text)); // Echo message back to sender
            }
            Ok(ws::Message::Close(_)) => {
                println!("Connection closed.");
                ctx.stop();
            }
            _ => (),
        }
    }
}

// WebSocket entry point
pub async fn ws_index(req: HttpRequest, stream: web::Payload) -> impl Responder {
    ws::start(WsChatSession {}, &req, stream)
}
