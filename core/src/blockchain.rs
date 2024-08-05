use k256::ecdsa::{Signature as EcdsaSignature, SigningKey, VerifyingKey};
use k256::ecdsa::signature::{Signer, Verifier};
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::block::{Block, Signature as BlockSignature};
use crate::transaction::Transaction;
use crate::db::Database;
use cryptography::schnorr::{sign_message, verify_message};
use zero_knowledge::proof::{ZkProver, verify_proof};
use zero_knowledge::trace::create_trace;
use winter_prover::Proof;
use winter_math::fields::f128::BaseElement;
use common::config::{INITIAL_DIFFICULTY, TARGET_BLOCK_TIME, MAX_BLOCK_SIZE, DIFFICULTY_ADJUSTMENT_WINDOW, MAXIMUM_SUPPLY};

#[derive(Clone, Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub target_block_time: u64,
    pub max_block_size: u64,
    pub difficulty_adjustment_window: u64,
    pub current_difficulty: u64,
    pub current_supply: u64, // Track the current supply
    pub db: Arc<Mutex<Database>>,
}

impl Blockchain {
    pub fn new(db_path: &str) -> Self {
        Self {
            blocks: vec![],
            target_block_time: TARGET_BLOCK_TIME,
            max_block_size: MAX_BLOCK_SIZE as u64,
            difficulty_adjustment_window: DIFFICULTY_ADJUSTMENT_WINDOW as u64,
            current_difficulty: INITIAL_DIFFICULTY,
            current_supply: 0, // Initialize current supply
            db: Arc::new(Mutex::new(Database::new(db_path))),
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.current_supply += block.reward; // Update the current supply with the new block reward
        self.blocks.push(block.clone());
        // Store block in the database
        self.db.lock().unwrap().store_block(&block).expect("Failed to store block");
    }

    pub fn adjust_difficulty(&self) -> u64 {
        let last_index = self.blocks.len() as u64 - 1;
        if last_index % self.difficulty_adjustment_window != 0 || last_index == 0 {
            return self.current_difficulty; // Current difficulty
        }

        let window_start = last_index - self.difficulty_adjustment_window;
        let window_end = last_index;

        let start_block = &self.blocks[window_start as usize];
        let end_block = &self.blocks[window_end as usize];

        let actual_time_taken = end_block.timestamp - start_block.timestamp;
        let expected_time_taken = self.target_block_time * self.difficulty_adjustment_window;

        let new_difficulty = (self.current_difficulty as u128
            * expected_time_taken as u128
            / actual_time_taken as u128) as u64;

        new_difficulty
    }

    pub fn mine_block(&mut self, miner: String, private_key: &SigningKey) -> Block {
        let previous_block = self.blocks.last().expect("Blockchain is empty");
        let index = previous_block.index + 1;
        let previous_hash = &previous_block.previous_hash;
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis();
        let nonce = 0;

        let base_reward = self.current_difficulty; // Define a base reward
        let reward = calculate_block_reward(self.current_supply, base_reward);

        let mut new_block = Block::new(
            index,
            previous_hash.clone(),
            timestamp,
            nonce,
            miner.clone(),
            BlockSignature(EcdsaSignature::from_bytes(&[0u8; 64].into()).unwrap()),
            reward,
            vec![],
        );
        new_block.sign(private_key);
        self.add_block(new_block.clone());

        new_block
    }

    pub fn get_balance(&self, address: &str) -> u64 {
        self.blocks.iter().fold(0, |acc, block| {
            acc + block.transactions.iter().fold(0, |acc, tx| {
                if tx.sender == address {
                    acc - tx.amount
                } else if tx.receiver == address {
                    acc + tx.amount
                } else {
                    acc
                }
            })
        })
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), String> {
        if self.get_balance(&transaction.sender) < transaction.amount {
            return Err("Insufficient balance".to_string());
        }
        // Normally you would add the transaction to a pool and include it in the next block
        Ok(())
    }

    pub fn verify_block(&self, block: &Block, public_key: &VerifyingKey) -> bool {
        let hash = Block::calculate_hash(
            block.index,
            &block.previous_hash,
            block.timestamp,
            block.nonce,
            &block.miner,
            &block.signature,
            block.reward,
        );
        let message = hash.as_bytes();
        verify_message(message, &block.signature.0, public_key)
    }
}

/// Calculates the block reward based on the current supply and the base reward.
fn calculate_block_reward(current_supply: u64, base_reward: u64) -> u64 {
    if current_supply + base_reward > MAXIMUM_SUPPLY {
        // Adjust reward to not exceed MAXIMUM_SUPPLY
        MAXIMUM_SUPPLY - current_supply
    } else {
        base_reward
    }
}
