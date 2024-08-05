// consensus/src/pos.rs

use crate::block::Block;
use crate::transaction::Transaction;
use cryptography::schnorr::sign_message;
use zero_knowledge::winterfell::{create_trace, ZkProver};
use winter_math::fields::f128::BaseElement;
use std::time::{SystemTime, UNIX_EPOCH};

impl Blockchain {
    pub fn mine_pos_block(&mut self, miner: String, stake_modifier: String, private_key: &SigningKey) -> Block {
        let previous_block = self.blocks.last().expect("Blockchain is empty");
        let index = previous_block.index + 1;
        let previous_hash = &previous_block.previous_hash;
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis();
        let nonce = 0;

        // Example transaction
        let sender = "sender_address".to_string();
        let receiver = "receiver_address".to_string();
        let amount = 100;
        let signature = sign_message(&amount.to_be_bytes(), private_key);

        // Create zero-knowledge proof for transaction
        let inputs = vec![BaseElement::new(amount as u128)];
        let trace = create_trace(inputs);
        let prover = ZkProver::new(trace);
        let zk_proof = prover.generate_proof().expect("Failed to generate proof").to_bytes();

        let transaction = Transaction::new(sender, receiver, amount, signature, zk_proof);

        let mut new_block = Block::new(index, previous_hash.clone(), timestamp, nonce, miner, vec![transaction]);
        self.add_block(new_block.clone());

        new_block
    }
}

