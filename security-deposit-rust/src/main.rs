use std::collections::HashMap;

// Define a struct to represent the Security Deposit
#[derive(Debug)]
struct SecurityDeposit {
    tenant_name: String,
    amount: f64,
    deposit_date: String,
    deductions: f64,
}


impl SecurityDeposit {
    // Create a new deposit
    fn new(tenant_name: &str, amount: f64, deposit_date: &str) -> SecurityDeposit {
        SecurityDeposit {
            tenant_name: tenant_name.to_string(),
            amount,
            deposit_date: deposit_date.to_string(),
            deductions: 0.0,
        }
    }
   // Add a deduction to the deposit
   fn add_deduction(&mut self, deduction: f64) {
    self.deductions += deduction;
}

// Calculate the refund after deductions
fn calculate_refund(&self) -> f64 {
    self.amount - self.deductions
}
}