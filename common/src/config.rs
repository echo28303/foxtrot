pub const INITIAL_TARGET_BLOCK_TIME: u64 = 8; // Initial target block time in seconds
pub const INITIAL_DIFFICULTY: u64 = 1;
pub const MAX_BLOCK_SIZE: usize = 1_000_000; // Maximum block size in bytes
pub const MAXIMUM_SUPPLY: u64 = 21_000_000; // Maximum coin supply

// Initial adjustment window during the launch phase
pub const INITIAL_ADJUSTMENT_WINDOW: usize = 10; 

// Normal adjustment window once stabilized
pub const ADJUSTMENT_WINDOW: usize = 100;

// Percentage change for adjusting the target block time
pub const BLOCK_TIME_ADJUSTMENT_PERCENT: u64 = 5; // Percentage (e.g., 5% increase or decrease)

// Network identifiers
#[cfg(feature = "mainnet")]
pub const NETWORK_ID: &str = "mainnet";

#[cfg(not(feature = "testnet"))]
pub const NETWORK_ID: &str = "testnet";
