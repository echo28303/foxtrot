pub const TARGET_BLOCK_TIME: u64 = 8; // Target block time in seconds
pub const INITIAL_DIFFICULTY: u64 = 1;
pub const MAX_BLOCK_SIZE: usize = 1_000_000; // Maximum block size in bytes
pub const DIFFICULTY_ADJUSTMENT_WINDOW: usize = 2016;
pub const MAXIMUM_SUPPLY: u64 = 21_000_000; // Maximum coin supply

#[cfg(feature = "mainnet")]
pub const NETWORK_ID: &str = "mainnet";

#[cfg(not(feature = "testnet"))]
pub const NETWORK_ID: &str = "testnet";
