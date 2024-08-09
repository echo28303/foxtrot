mod pos;
mod pow;
mod transition;

use pos::{Blockchain, Block};
use transition::TransitionManager;
use std::sync::{Arc, Mutex};
use cryptography::schnorr::generate_keypair;
use common::config::MAXIMUM_SUPPLY;

fn main() {
    // Initialize the blockchain
    let blockchain = Arc::new(Mutex::new(Blockchain::new("./db")));

    // Generate keypair for signing
    let (private_key, _public_key) = generate_keypair();

    // Create the transition manager
    let transition_manager = TransitionManager::new(blockchain.clone(), private_key);

    // Mining loop
    loop {
        transition_manager.manage_transition();

        // Optionally, add sleep or break conditions here

        // Check if maximum supply has been reached and exit loop if true
        let blockchain = blockchain.lock().unwrap();
        if blockchain.current_supply >= MAXIMUM_SUPPLY {
            break;
        }
    }

    // Print the current state of the blockchain
    let blockchain = blockchain.lock().unwrap();
    for block in &blockchain.blocks {
        println!("{:?}", block);
    }
}
