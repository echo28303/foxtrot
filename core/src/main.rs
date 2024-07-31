mod config;
mod blockchain;
mod transaction;
mod block;
mod account;
mod db;

use blockchain::Blockchain;
use transaction::Transaction;
use block::Block;
use k256::ecdsa::{SigningKey, VerifyingKey, signature::Signer, signature::Verifier};

fn main() {
    let db_path = "./db";
    let mut blockchain = Blockchain::new(db_path);
    println!("Blockchain created with network ID: {}", config::NETWORK_ID);

    // Example private and public keys
    // Replace these with actual keys
    let private_key_bytes = [0u8; 32];
    let private_key = SigningKey::from_bytes(&private_key_bytes).expect("Failed to create private key");
    let public_key = VerifyingKey::from(&private_key);

    // Create and sign a transaction
    let tx = Transaction::new(
        String::from("sender_address"),
        String::from("receiver_address"),
        100,
        &private_key,
    );

    // Add the transaction to the blockchain
    blockchain.add_transaction(tx.clone()).unwrap();

    // Mine a new block
    blockchain.mine_block(String::from("miner_address"), &private_key);

    // Check balances
    let sender_balance = blockchain.get_balance("sender_address");
    let receiver_balance = blockchain.get_balance("receiver_address");
    println!("Sender balance: {}", sender_balance);
    println!("Receiver balance: {}", receiver_balance);

    // Create a test block
    let test_block = Block::new(
        1,
        String::from("previous_hash"),
        1234567890,
        10,
        String::from("miner_address"),
        vec![tx],
    );
    let calculated_hash = test_block.calculate_hash();
    println!("Calculated Hash: {}", calculated_hash);

    // Sign and verify the block
    let mut signed_block = test_block.clone();
    signed_block.sign(&private_key);

    let is_signature_valid = signed_block.verify_signature(&public_key);
    println!("Is Signature Valid: {}", is_signature_valid);
}
