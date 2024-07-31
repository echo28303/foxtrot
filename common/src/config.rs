// src/config.rs
pub const TARGET_BLOCK_TIME: u64 = 8; // Target block time in seconds
pub const TRANSITION_BLOCK: u64 = 125_000_000_000_000;
pub const INITIAL_DIFFICULTY: u64 = 1;
pub const MAX_BLOCK_SIZE: usize = 1_000_000; // Maximum block size in bytes
pub const DIFFICULTY_ADJUSTMENT_WINDOW: usize = 2016;
pub const NETWORK_ID: &str = "testnet";
