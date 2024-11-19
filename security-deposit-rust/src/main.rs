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


fn main() {
    // A HashMap to track multiple tenants and their security deposits
    let mut deposits: HashMap<String, SecurityDeposit> = HashMap::new();

    // Add a new deposit for a tenant
    let deposit1 = SecurityDeposit::new("John Doe", 1000.0, "2024-11-01");
    deposits.insert(deposit1.tenant_name.clone(), deposit1);

    let deposit2 = SecurityDeposit::new("Jane Smith", 1200.0, "2024-11-05");
    deposits.insert(deposit2.tenant_name.clone(), deposit2);

    // Add a deduction to a tenant's deposit
    if let Some(deposit) = deposits.get_mut("John Doe") {
        deposit.add_deduction(200.0); // Deduction due to damages
    }

      // Calculate and display refunds for each tenant
      for (tenant, deposit) in &deposits {
        println!(
            "{}'s security deposit: ${}, Refund after deductions: ${}",
            tenant,
            deposit.amount,
            deposit.calculate_refund()
        );
    }
  
}