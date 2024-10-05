use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};

// Define the structure of a Block
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Block {
    index: u64,
    timestamp: u128,
    data: String,
    previous_hash: String,
    hash: String,
}

impl Block {
    // Generate a new block
    fn new(index: u64, data: String, previous_hash: String) -> Block {
        let timestamp = current_timestamp();
        let hash = calculate_hash(index, timestamp, &data, &previous_hash);
        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }
}

// Get the current timestamp
fn current_timestamp() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
}

// Hashing function (for simplicity, we'll use a simple hash placeholder)
fn calculate_hash(index: u64, timestamp: u128, data: &str, previous_hash: &str) -> String {
    format!("{:x}", md5::compute(format!("{}{}{}{}", index, timestamp, data, previous_hash)))
}

// Initialize the blockchain with a genesis block
fn create_genesis_block() -> Block {
    Block::new(0, "Genesis Block".to_string(), "0".to_string())
}

// A simple blockchain structure
struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    // Create a new blockchain with only the genesis block
    fn new() -> Blockchain {
        Blockchain {
            chain: vec![create_genesis_block()],
        }
    }

    // Get the latest block
    fn latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    // Add a new block to the chain
    fn add_block(&mut self, data: String) {
        let latest_block = self.latest_block();
        let new_block = Block::new(
            latest_block.index + 1,
            data,
            latest_block.hash.clone(),
        );
        self.chain.push(new_block);
    }
}

fn main() {
    // Initialize a new blockchain
    let mut blockchain = Blockchain::new();

    // Add a few blocks
    blockchain.add_block("First block after Genesis".to_string());
    blockchain.add_block("Second block".to_string());

    // Display the blockchain
    for block in &blockchain.chain {
        println!("{:?}", block);
    }
}
