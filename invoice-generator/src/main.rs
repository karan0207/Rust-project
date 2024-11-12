use std::fs::File;
use std::io::{self, Write};
use std::fmt;

struct InvoiceItem {
    description: String,
    quantity: u32,
    unit_price: f64,
}

impl InvoiceItem {
    fn total(&self) -> f64 {
        self.quantity as f64 * self.unit_price
    }