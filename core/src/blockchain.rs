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
use common::config::{
    INITIAL_DIFFICULTY, 
    INITIAL_TARGET_BLOCK_TIME, 
    MAX_BLOCK_SIZE, 
    MAXIMUM_SUPPLY, 
    INITIAL_ADJUSTMENT_WINDOW, 
    ADJUSTMENT_WINDOW, 
    BLOCK_TIME_ADJUSTMENT_PERCENT,
};

#[derive(Clone, Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub target_block_time: u64,
    pub max_block_size: u64,
    pub current_difficulty: u64,
    pub current_supply: u64, // Track the current supply
    pub db: Arc<Mutex<Database>>,
}

impl Blockchain {
    pub fn new(db_path: &str) -> Self {
        Self {
            blocks: vec![],
            target_block_time: INITIAL_TARGET_BLOCK_TIME,
            max_block_size: MAX_BLOCK_SIZE as u64,
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
        // Adjust difficulty and block time after adding a block
        self.adjust_difficulty_and_block_time();
    }

    pub fn adjust_difficulty_and_block_time(&mut self) {
        let window_size = if self.blocks.len() < INITIAL_ADJUSTMENT_WINDOW {
            INITIAL_ADJUSTMENT_WINDOW // Use initial window size during the launch phase
        } else {
            ADJUSTMENT_WINDOW // Normal window size once stabilized
        };

        let last_index = self.blocks.len() as u64 - 1;
        if last_index == 0 || last_index % window_size as u64 != 0 {
            return; // No adjustment if it's not the end of a window
        }

        let window_start = (last_index - window_size as u64 + 1) as usize;
        let window_end = last_index as usize;

        let start_block = &self.blocks[window_start];
        let end_block = &self.blocks[window_end];

        let actual_time_taken = end_block.timestamp - start_block.timestamp;
        let expected_time_taken = self.target_block_time * window_size as u64;

        // Adjust difficulty
        self.current_difficulty = (self.current_difficulty as u128
            * expected_time_taken as u128
            / actual_time_taken as u128) as u64;

        // Adjust block time based on propagation
        self.target_block_time = self.adjust_block_time(actual_time_taken, expected_time_taken);
    }

    pub fn adjust_block_time(&self, actual_time_taken: u64, expected_time_taken: u64) -> u64 {
        let percent_change = BLOCK_TIME_ADJUSTMENT_PERCENT as f64 / 100.0;

        if actual_time_taken > expected_time_taken {
            // Increase block time
            ((self.target_block_time as f64) * (1.0 + percent_change)) as u64
        } else {
            // Decrease block time
            ((self.target_block_time as f64) * (1.0 - percent_change)) as u64
        }
    }

    pub fn mine_block(&mut self, miner: String, private_key: &SigningKey) -> Block {
        let previous_block = self.blocks.last().expect("Blockchain is empty");
        let index = previous_block.index + 1;
        let previous_hash = &previous_block.previous_hash;
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis() as u64;
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
