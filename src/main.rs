use core::block::{Block, Signature};
use core::blockchain::Blockchain;
use core::transaction::Transaction;
use k256::ecdsa::{signature::Signer, SigningKey, VerifyingKey};
use k256::elliptic_curve::generic_array::GenericArray;
use typenum::U32;
use common::config;
use rand::rngs::OsRng;
use zero_knowledge::proof::{ZkProver, create_trace, generate_blake3_hash};
use winter_math::fields::f128::BaseElement;
use blake3::Hasher;

fn main() {
    let private_key_bytes: GenericArray<u8, U32> = GenericArray::clone_from_slice(&[0u8; 32]);
    let private_key = SigningKey::from_bytes(&private_key_bytes).expect("Failed to create private key");

    let db_path = "./db";
    let mut blockchain = Blockchain::new(db_path);
    println!("Blockchain created with network ID: {}", config::NETWORK_ID);

    // Example private and public keys
    let private_key = SigningKey::random(&mut OsRng);
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

    // Generate a Blake3 hash for trace
    let trace = create_trace(vec![BaseElement::new(100)]);
    let hash = generate_blake3_hash(&trace.to_bytes());
    println!("Generated Blake3 Hash: {:?}", hash);
}
