#[cfg(test)]
mod tests {
    use super::*;
    use cryptography::schnorr::generate_keypair;
    use k256::ecdsa::SigningKey;

    #[test]
    fn test_transaction_hashing() {
        let sender = String::from("sender_address");
        let receiver = String::from("receiver_address");
        let amount = 100;
        let fee = 1;
        let transaction = Transaction::new(sender.clone(), receiver.clone(), amount, fee);
        let expected_hash = transaction.hash();
        assert_eq!(transaction.hash(), expected_hash);
    }

    #[test]
    fn test_transaction_signing() {
        let (private_key, public_key) = generate_keypair();
        let sender = String::from("sender_address");
        let receiver = String::from("receiver_address");
        let amount = 100;
        let fee = 1;
        let mut transaction = Transaction::new(sender, receiver, amount, fee);
        transaction.sign(&private_key);
        assert!(transaction.verify_signature(&public_key));
    }
}

