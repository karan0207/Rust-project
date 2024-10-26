use rand::{distributions::Alphanumeric, Rng};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

// Mock payment structure
struct Payment {
    transaction_id: String,
    amount: f64,
    verified: bool,
}

impl Payment {
    fn new(amount: f64) -> Payment {
        Payment {
            transaction_id: generate_transaction_id(),
            amount,
            verified: false,
        }
    }
}

// Generates a unique transaction ID
fn generate_transaction_id() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect()
}

// Verifies a transaction with a token
fn verify_transaction(payment: &mut Payment, token: &str) -> bool {
    let computed_token = create_verification_token(&payment.transaction_id);
    if token == computed_token {
        payment.verified = true;
        true
    } else {
        false
    }
}

// Creates a hash-based verification token for a transaction
fn create_verification_token(transaction_id: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(transaction_id);
    format!("{:x}", hasher.finalize())
}

fn main() {
    // Step 1: Create a new payment
    let mut payment = Payment::new(150.0);
    println!("Created Payment:");
    println!("Transaction ID: {}", payment.transaction_id);
    println!("Amount: ${}", payment.amount);

    // Step 2: Generate a verification token for the transaction
    let token = create_verification_token(&payment.transaction_id);
    println!("\nVerification Token: {}", token);

    // Step 3: Verify the transaction
    let verification_result = verify_transaction(&mut payment, &token);
    if verification_result {
        println!("\nPayment verified successfully!");
    } else {
        println!("\nPayment verification failed!");
    }
}
