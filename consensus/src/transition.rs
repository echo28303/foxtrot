use crate::pos::Blockchain;
use crate::pow::ProofOfWork;
use std::sync::{Arc, Mutex};
use k256::ecdsa::SigningKey;
use common::config::MAXIMUM_SUPPLY;

/// Manages the transition from Proof of Work (PoW) to Proof of Stake (PoS).
pub struct TransitionManager {
    blockchain: Arc<Mutex<Blockchain>>,
    private_key: SigningKey,
}

impl TransitionManager {
    /// Creates a new TransitionManager.
    pub fn new(blockchain: Arc<Mutex<Blockchain>>, private_key: SigningKey) -> Self {
        Self {
            blockchain,
            private_key,
        }
    }

    /// Checks if the transition from PoW to PoS should occur and performs the transition.
    pub fn manage_transition(&self) {
        let mut blockchain = self.blockchain.lock().unwrap();

        if blockchain.current_supply >= MAXIMUM_SUPPLY {
            // Transition to Proof of Stake
            let miner_address = "miner_address".to_string();
            let stake_modifier = "stake_modifier".to_string();

            let new_block = blockchain.mine_pos_block(miner_address, stake_modifier, &self.private_key);
            blockchain.add_block(new_block);
        } else {
            // Continue with Proof of Work
            let miner_address = "miner_address".to_string();
            let proof_of_work = ProofOfWork::new(self.blockchain.clone());
            let new_block = proof_of_work.mine_block(miner_address, &self.private_key);
            blockchain.add_block(new_block);
        }
    }
}
