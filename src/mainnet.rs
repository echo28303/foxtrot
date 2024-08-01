use core::blockchain::Blockchain;
use common::config;

fn main() {
    // Load configuration from environment variables or configuration files
    let db_path = std::env::var("DB_PATH").unwrap_or_else(|_| "./db".to_string());
    let network_id = std::env::var("NETWORK_ID").unwrap_or_else(|_| config::NETWORK_ID.to_string());

    // Initialize the blockchain
    let mut blockchain = Blockchain::new(&db_path);
    println!("Blockchain created with network ID: {}", network_id);

    // Additional initialization logic for production
    // e.g., connecting to peers, starting network services, etc.
    
    // Start the blockchain node
   // blockchain.start();

    // Handle shutdown and cleanup
    // ...
}

