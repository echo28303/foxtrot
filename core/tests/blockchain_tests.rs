// core/tests/blockchain_tests.rs
use core::blockchain::Blockchain;
use core::transaction::Transaction;

#[test]
fn test_add_transaction() {
    let mut blockchain = Blockchain::new(
        "./db",
        10, // block_reward
        1,  // initial_difficulty
        1_000_000, // max_block_size
        8, // block_time (in seconds)
        2016, // difficulty_adjustment_window (number of blocks)
        String::from("testnet"),
    );

    let private_key = "your_private_key_here";
    let public_key = "your_public_key_here";

    let mut tx = Transaction::new(String::from("sender_address"), String::from("receiver_address"), 100, String::new());
    tx.sign(private_key);

    assert!(blockchain.add_transaction(tx.clone()).is_ok());
}

#[test]
fn test_mine_block() {
    let mut blockchain = Blockchain::new(
        "./db",
        10, // block_reward
        1,  // initial_difficulty
        1_000_000, // max_block_size
        8, // block_time (in seconds)
        2016, // difficulty_adjustment_window (number of blocks)
        String::from("testnet"),
    );

    let private_key = "your_private_key_here";
    let public_key = "your_public_key_here";

    let mut tx = Transaction::new(String::from("sender_address"), String::from("receiver_address"), 100, String::new());
    tx.sign(private_key);

    blockchain.add_transaction(tx.clone()).unwrap();
    assert!(blockchain.mine_block(String::from("miner_address"), private_key).is_ok());
}

#[test]
fn test_store_and_retrieve_block() {
    let mut blockchain = Blockchain::new(
        "./db",
        10, // block_reward
        1,  // initial_difficulty
        1_000_000, // max_block_size
        8, // block_time (in seconds)
        2016, // difficulty_adjustment_window (number of blocks)
        String::from("testnet"),
    );

    let private_key = "your_private_key_here";
    let public_key = "your_public_key_here";

    let mut tx = Transaction::new(String::from("sender_address"), String::from("receiver_address"), 100, String::new());
    tx.sign(private_key);

    blockchain.add_transaction(tx.clone()).unwrap();
    blockchain.mine_block(String::from("miner_address"), private_key).unwrap();

    let block = blockchain.get_block(1).expect("Block should be present");
    assert_eq!(block.index, 1);
}

#[test]
fn test_store_and_retrieve_account() {
    let mut blockchain = Blockchain::new(
        "./db",
        10, // block_reward
        1,  // initial_difficulty
        1_000_000, // max_block_size
        8, // block_time (in seconds)
        2016, // difficulty_adjustment_window (number of blocks)
        String::from("testnet"),
    );

    let account = core::account::Account::new(String::from("account_address"), 1000);
    blockchain.store_account(account.clone());

    let retrieved_account = blockchain.get_account("account_address").expect("Account should be present");
    assert_eq!(retrieved_account.address, account.address);
    assert_eq!(retrieved_account.balance, account.balance);
}

