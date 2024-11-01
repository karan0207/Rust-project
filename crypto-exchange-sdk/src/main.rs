use crypto_exchange_sdk::{Client, OrderType, Side};
use std::env;

#[tokio::main]
async fn main() {
    // Fetch API credentials from environment variables (example setup)
    let api_key = env::var("API_KEY").expect("API_KEY not set");
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY not set");

    // Initialize the SDK client
    let client = Client::new(api_key, secret_key);

    // Example: Fetch the latest market data for BTC-USD
    match client.get_market_data("BTC-USD").await {
        Ok(data) => println!("Market data: {:?}", data),
        Err(e) => eprintln!("Error fetching market data: {}", e),
    }

    // Example: Place a limit order
    match client.place_order("BTC-USD", Side::Buy, OrderType::Limit, 0.01, 50000.0).await {
        Ok(order) => println!("Order placed: {:?}", order),
        Err(e) => eprintln!("Error placing order: {}", e),
    }
}
