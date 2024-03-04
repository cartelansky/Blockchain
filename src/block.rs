use chrono::prelude::*;

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain { chain: vec![] }
    }
}

#[derive(Debug)]
pub struct Block {
    index: u64,
    timestamp: DateTime<Utc>,
    transaction: Vec<Transaction>,
    pub hash: String,
    previous_hash: String,
    nonce: u64,
}

impl Block {
    pub fn new(index: u64, transaction: Vec<Transaction>, previous_hash: String) -> Block {
        let timestamp = Utc::now();
        let mut nonce = 0;
        let mut hash = String::new();

        loop {
            hash = sha256::digest(format!(
                "{timestamp}{previous_hash}{:?}{nonce}",
                transaction
            ));

            if hash.starts_with("000") {
                break;
            }

            nonce += 1;
        }

        Block {
            index,
            timestamp,
            transaction,
            hash,
            previous_hash,
            nonce,
        }
    }
}

#[derive(Debug)]
pub struct Transaction {
    sender: String,
    receive: String,
    amount: u64,
}

impl Transaction {
    pub fn new(sender: String, receive: String, amount: u64) -> Transaction {
        Transaction {
            sender,
            receive,
            amount,
        }
    }
}
