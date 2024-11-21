use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;


#[derive(Debug, serde::Deserialize)]
struct StockData {
    Date: String,
    Open: f64,
    High: f64,
    Low: f64,
    Close: f64,
    Volume: u64,
}


/// Calculate the moving average of a given window size
fn moving_average(prices: &[f64], window: usize) -> Vec<f64> {
    prices
        .windows(window)
        .map(|w| w.iter().sum::<f64>() / window as f64)
        .collect()
}



/// Simulate the moving average crossover strategy
fn moving_average_crossover(prices: &[f64], short_window: usize, long_window: usize) {
    let short_ma = moving_average(prices, short_window);
    let long_ma = moving_average(prices, long_window);

    let mut position = false; // False = No position, True = Holding stock
    let mut profit = 0.0;
    let mut buy_price = 0.0;

    for i in long_window..prices.len() {
        let short_value = short_ma[i - long_window];
        let long_value = long_ma[i - long_window];

        if short_value > long_value && !position {
            // Buy signal
            position = true;
            buy_price = prices[i];
            println!("Buy at ${:.2} on day {}", buy_price, i);
        } else if short_value < long_value && position {
            // Sell signal
            position = false;
            let sell_price = prices[i];
            profit += sell_price - buy_price;
            println!("Sell at ${:.2} on day {} (Profit: ${:.2})", sell_price, i, sell_price - buy_price);
        }
    }

    println!("Total Profit: ${:.2}", profit);
}