#[cfg(test)]
mod tests {
    use super::*;
    use cryptography::schnorr::generate_keypair;
    use k256::ecdsa::SigningKey;

    #[test]
    fn test_block_hashing() {
        let index = 0;
        let previous_hash = String::from("00000000000000000000000000000000");
        let timestamp = 1627847289u128;
        let nonce = 0;
        let miner = String::from("miner_address");
        let signature = String::new();
        let reward = 50;

        let block = Block::new(index, previous_hash, timestamp, nonce, miner, signature, reward);
        let expected_hash = Block::calculate_hash(index, "00000000000000000000000000000000", timestamp, nonce, "miner_address", "", reward);
        assert_eq!(block.hash, expected_hash);
    }

    #[test]
    fn test_block_signing() {
        let (private_key, public_key) = generate_keypair();
        let index = 0;
        let previous_hash = String::from("00000000000000000000000000000000");
        let timestamp = 1627847289u128;
        let nonce = 0;
        let miner = String::from("miner_address");
        let signature = String::new();
        let reward = 50;

        let mut block = Block::new(index, previous_hash, timestamp, nonce, miner, signature, reward);
        block.sign(&private_key);
        assert!(block.verify_signature(&public_key));
    }
}

