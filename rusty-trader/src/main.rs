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