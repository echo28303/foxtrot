mod pos;
mod pow;
mod transition;

use pos::{Blockchain, Block};
use pow::ProofOfWork;
use std::sync::{Arc, Mutex};
use foxtrot::config::{
    TARGET_BLOCK_TIME, TRANSITION_BLOCK, INITIAL_DIFFICULTY, MAX_BLOCK_SIZE,
    DIFFICULTY_ADJUSTMENT_WINDOW,
};
use cryptography::schnorr::{generate_keypair, sign_message, verify_message};
use zero_knowledge::winterfell::{ZkProver, create_trace, verify_proof};
use winterfell::math::fields::f128::BaseElement;

fn main() {
    // Initialize the blockchain
    let blockchain = Arc::new(Mutex::new(Blockchain::new(
        TARGET_BLOCK_TIME,
        TRANSITION_BLOCK,
        MAX_BLOCK_SIZE as u64, // Convert to u64
        DIFFICULTY_ADJUSTMENT_WINDOW as u64, // Convert to u64
        "./db",
    )));

    // Example miner details
    let miner_address = "miner_address".to_string();
    let stake_modifier = "stake_modifier".to_string();

    // Generate keypair for signing
    let (private_key, public_key) = generate_keypair();

    {
        let mut blockchain = blockchain.lock().unwrap();

        // Mine blocks until the transition block is reached
        while blockchain.blocks.len() as u64 <= TRANSITION_BLOCK {
            // PoW phase
            if blockchain.blocks.len() as u64 < TRANSITION_BLOCK {
                let proof_of_work = ProofOfWork::new(blockchain.clone());
                let new_block = proof_of_work.mine_block(miner_address.clone(), &private_key);
                blockchain.add_block(new_block);
            } else {
                // PoS phase
                let new_block = blockchain.mine_pos_block(miner_address.clone(), stake_modifier.clone(), &private_key);
                blockchain.add_block(new_block);

                // Example inputs for the trace for zero-knowledge proof
                let inputs = vec![BaseElement::new(1), BaseElement::new(2), BaseElement::new(3)];

                // Create a trace table from inputs
                let trace = create_trace(inputs);

                // Create a prover with the trace table
                let prover = ZkProver::new(trace);

                // Generate a zero-knowledge proof
                let proof = prover.generate_proof().expect("Failed to generate proof");

                // Verify the proof
                verify_proof(proof).expect("Proof verification failed");
            }
        }
    }

    // Print the current state of the blockchain
    let blockchain = blockchain.lock().unwrap();
    for block in &blockchain.blocks {
        println!("{:?}", block);
    }
}

