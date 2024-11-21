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