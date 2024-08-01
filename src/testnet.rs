use core::block::{Block, Signature};
use core::blockchain::Blockchain;
use core::transaction::Transaction;
use common::config;
use zero_knowledge::proof::ZkProver;
use winter_math::fields::f128::BaseElement;
use cryptography::schnorr::{generate_keypair, sign_message, verify_message};
use zero_knowledge::trace::{create_trace}; // Import functions from trace.rs

fn main() {
    // Generate a real key pair using Schnorr signatures
    let (private_key, public_key) = generate_keypair();

    let db_path = "./db";
    let mut blockchain = Blockchain::new(db_path);
    println!("Blockchain created with network ID: {}", config::NETWORK_ID);

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
        Signature::from_slice(&[0u8; 64]),  // Use from_slice
        0,
        vec![tx.clone()],
    );

    let calculated_hash = Block::calculate_hash(
        1,
        "previous_hash",
        1234567890,
        10,
        "miner_address",
        &Signature::from_slice(&[0u8; 64]),  // Use from_slice
        0
    );

    println!("Calculated Hash: {}", calculated_hash);

    // Sign and verify the block
    let mut signed_block = test_block.clone();
    signed_block.sign(&private_key);

    let is_signature_valid = signed_block.verify_signature(&public_key);
    println!("Is Signature Valid: {}", is_signature_valid);

    // Generate and use the raw trace
    let trace = create_trace(vec![
        BaseElement::new(100),
        BaseElement::new(200),
        BaseElement::new(300),
    ]); // Ensure trace has at least 8 elements by padding with zeros in create_trace
    println!("Generated Trace: {:?}", trace);
}
