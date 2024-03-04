mod block;

use block::{Block, Blockchain, Transaction};

fn main() {
    let mut blockchain = Blockchain::new();

    let genesis_block = Block::new(0, vec![], String::new());
    blockchain.chain.push(genesis_block);

    let tx1 = Transaction::new("Ali Koc".to_string(), "Aziz Yıldırım".to_string(), 1000000);

    let first_block = Block::new(1, vec![tx1], blockchain.chain[0].hash.clone());
    blockchain.chain.push(first_block);

    blockchain.chain.push(Block::new(
        2,
        vec![Transaction::new(
            String::from("Gavin Wood"),
            String::from("Vitalik Buterin"),
            7000000,
        )],
        blockchain.chain[1].hash.clone(),
    ));

    let tx2 = Transaction::new(
        "Kendall Roy".to_string(),
        String::from("Logan Roy"),
        999999999,
    );

    let second_block = Block::new(3, vec![tx2], blockchain.chain[2].hash.clone());
    blockchain.chain.push(second_block);

    blockchain
        .chain
        .push(Block::new(4, vec![], blockchain.chain[3].hash.clone()));
    println!("{:#?}", blockchain);
}
