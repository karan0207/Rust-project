use std::collections::HashMap;

// Structure to hold token balances
struct TokenSystem {
    balances: HashMap<String, u128>, // Account balances
}

impl TokenSystem {
    // Constructor for creating the system
    fn new() -> TokenSystem {
        TokenSystem {
            balances: HashMap::new(),
        }
    }

    // Mint new tokens
    fn mint(&mut self, account: String, amount: u128) {
        let balance = self.balances.entry(account.clone()).or_insert(0);
        *balance += amount;
        println!("Minted {} tokens to {}!", amount, account);
    }

    // Transfer tokens between accounts
    fn transfer(&mut self, from: String, to: String, amount: u128) -> Result<(), String> {
        let sender_balance = self.balances.entry(from.clone()).or_insert(0);

        if *sender_balance < amount {
            return Err(format!("Insufficient balance in {}'s account", from));
        }

        *sender_balance -= amount;

        let receiver_balance = self.balances.entry(to.clone()).or_insert(0);
        *receiver_balance += amount;

        println!("Transferred {} tokens from {} to {}!", amount, from, to);
        Ok(())
    }

    // Check balance of a specific account
    fn balance_of(&self, account: &String) -> u128 {
        *self.balances.get(account).unwrap_or(&0)
    }
}

fn main() {
    let mut token_system = TokenSystem::new();

    // Mint tokens
    token_system.mint("Alice".to_string(), 1000);
    token_system.mint("Bob".to_string(), 500);

    // Transfer tokens
    token_system.transfer("Alice".to_string(), "Bob".to_string(), 200).unwrap();

    // Check balances
    println!("Balance of Alice: {}", token_system.balance_of(&"Alice".to_string()));
    println!("Balance of Bob: {}", token_system.balance_of(&"Bob".to_string()));
}
