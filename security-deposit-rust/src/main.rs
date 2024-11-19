use std::collections::HashMap;

// Define a struct to represent the Security Deposit
#[derive(Debug)]
struct SecurityDeposit {
    tenant_name: String,
    amount: f64,
    deposit_date: String,
    deductions: f64,
}
