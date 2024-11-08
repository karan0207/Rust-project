Real-Time Messaging Microservice in Rust
A WebSocket-based real-time messaging microservice built with Rust, Actix-Web, and Redis Pub/Sub. This microservice enables bidirectional messaging for connected clients and supports scalability through Redis message broadcasting.

Features
WebSocket Connections: Real-time, bidirectional communication with clients.
Redis Pub/Sub: Enables message broadcasting across multiple service instances.
Asynchronous I/O: Uses Tokio and Actix for efficient connection handling.
Tech Stack
Rust: Core language for this project.
Actix-Web: Framework for handling HTTP and WebSocket connections.
Redis: Used for message broadcasting through Pub/Sub.
Tokio: Asynchronous runtime for concurrency.
Getting Started
Prerequisites
Rust
Redis (for Pub/Sub functionality)
Installation
Clone the repository:

bash
Copy code
git clone https://github.com/your-username/real_time_messaging_service.git
cd real_time_messaging_service