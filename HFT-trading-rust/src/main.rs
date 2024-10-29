use chrono::Local;
use rand::Rng;
use std::{thread, time::Duration};

#[derive(Debug)]
struct Trade {
    timestamp: String,
    trade_type: String,
    price: f64,
    volume: f64,
}

// Generate mock market data with fluctuating prices
fn generate_market_data() -> f64 {
    let mut rng = rand::thread_rng();
    100.0 + rng.gen_range(-1.0..1.0)  // Prices fluctuate around 100
}

// Moving Average Crossover strategy
fn moving_average_crossover(prices: &Vec<f64>) -> Option<&'static str> {
    let short_window = 5;
    let long_window = 20;

    if prices.len() < long_window {
        return None; // Not enough data
    }

    let short_avg: f64 = prices.iter().rev().take(short_window).sum::<f64>() / short_window as f64;
    let long_avg: f64 = prices.iter().rev().take(long_window).sum::<f64>() / long_window as f64;

    if short_avg > long_avg {
        Some("BUY")
    } else if short_avg < long_avg {
        Some("SELL")
    } else {
        None
    }
}

// Execute a trade based on the strategy signal
fn execute_trade(price: f64, trade_type: &str) -> Trade {
    let volume = 1.0; // Fixed volume for simplicity
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    Trade {
        timestamp,
        trade_type: trade_type.to_string(),
        price,
        volume,
    }
}

fn main() {
    let mut price_history: Vec<f64> = Vec::new();
    let mut trades: Vec<Trade> = Vec::new();

    loop {
        let current_price = generate_market_data();
        price_history.push(current_price);

        if let Some(signal) = moving_average_crossover(&price_history) {
            let trade = execute_trade(current_price, signal);
            trades.push(trade);
        }

        println!("Current Price: {:.2}", current_price);
        for trade in &trades {
            println!("Executed Trade: {:?}", trade);
        }

        // Pause briefly to simulate high-frequency trading
        thread::sleep(Duration::from_millis(500));
    }
}
