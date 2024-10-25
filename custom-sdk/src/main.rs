use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Block {
    index: u64,
    previous_hash: String,
    timestamp: u128,
    transactions: Vec<Transaction>,
    hash: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: u64,
}

struct Blockchain {
    blocks: Vec<Block>,
    current_transactions: Vec<Transaction>,
}

impl Blockchain {
    fn new() -> Self {
        let mut blockchain = Blockchain {
            blocks: Vec::new(),
            current_transactions: Vec::new(),
        };
        blockchain.create_genesis_block();
        blockchain
    }

    fn create_genesis_block(&mut self) {
        let genesis_block = Block {
            index: 0,
            previous_hash: String::from("0"),
            timestamp: chrono::Utc::now().timestamp_millis() as u128,
            transactions: vec![],
            hash: String::from("genesis"),
        };
        self.blocks.push(genesis_block);
    }

    fn new_block(&mut self) {
        let previous_block = self.blocks.last().expect("Blockchain must have a genesis block");
        let previous_hash = previous_block.hash.clone();
        
        let block = Block {
            index: self.blocks.len() as u64,
            previous_hash,
            timestamp: chrono::Utc::now().timestamp_millis() as u128,
            transactions: self.current_transactions.clone(),
            hash: String::new(),
        };
        
        self.blocks.push(block);
        self.current_transactions.clear();
    }

    fn new_transaction(&mut self, sender: String, receiver: String, amount: u64) {
        let transaction = Transaction {
            sender,
            receiver,
            amount,
        };
        self.current_transactions.push(transaction);
    }
}
