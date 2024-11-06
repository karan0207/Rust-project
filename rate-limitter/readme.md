Rate-Limited API Client in Rust
This project demonstrates how to implement a simple rate-limited API client in Rust using tokio and reqwest. It allows you to make concurrent API requests while respecting a rate limit (e.g., 5 requests per second).

Features
Rate limit API requests to a specified number of requests per second.
Use of tokio async runtime for concurrent request handling.
Basic example to demonstrate rate-limited API calls using a semaphore.
Prerequisites
Rust (version 1.60 or higher)
Cargo (Rust's package manager and build system)
An async runtime (tokio) and an HTTP client library (reqwest)