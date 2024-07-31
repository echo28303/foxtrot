use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use core::blockchain::Blockchain;
use core::transaction::Transaction;
use core::block::Block;
use std::sync::Mutex;

#[derive(Deserialize)]
struct SendTransactionRequest {
    sender: String,
    receiver: String,
    amount: u64,
    private_key: String,
}

#[derive(Deserialize)]
struct TransactionFeeRequest {
    sender: String,
    receiver: String,
    amount: u64,
}

#[derive(Serialize)]
struct AccountResponse {
    address: String,
    balance: u64,
}

#[derive(Serialize)]
struct BlockResponse {
    index: u64,
    previous_hash: String,
    timestamp: u64,
    nonce: u64,
    miner: String,
    signature: String,
    reward: u64,
    transactions: Vec<Transaction>,
    hash: String,
}

#[derive(Serialize)]
struct BlockchainInfoResponse {
    current_height: u64,
    current_difficulty: u64,
    pending_transactions: usize,
    total_accounts: usize,
}

#[derive(Serialize)]
struct PendingTransactionsResponse {
    transactions: Vec<Transaction>,
}

#[derive(Serialize)]
struct TransactionFeeResponse {
    fee: u64,
}

#[derive(Deserialize)]
struct SubmitBlockRequest {
    index: u64,
    previous_hash: String,
    timestamp: u64,
    nonce: u64,
    miner: String,
    signature: String,
    reward: u64,
    transactions: Vec<Transaction>,
    hash: String,
}

#[derive(Serialize)]
struct AccountHistoryResponse {
    transactions: Vec<Transaction>,
}

pub async fn send_transaction(data: web::Data<Mutex<Blockchain>>, req: web::Json<SendTransactionRequest>) -> impl Responder {
    let mut blockchain = data.lock().unwrap();

    let mut transaction = Transaction::new(req.sender.clone(), req.receiver.clone(), req.amount, 1, String::new());
    transaction.sign(&req.private_key);

    match blockchain.add_transaction(transaction) {
        Ok(_) => HttpResponse::Ok().body("Transaction sent"),
        Err(e) => HttpResponse::BadRequest().body(e),
    }
}

pub async fn get_account(data: web::Data<Mutex<Blockchain>>, path: web::Path<String>) -> impl Responder {
    let blockchain = data.lock().unwrap();
    let address = path.into_inner();

    match blockchain.get_account(&address) {
        Some(account) => HttpResponse::Ok().json(AccountResponse {
            address: account.address,
            balance: account.balance,
        }),
        None => HttpResponse::NotFound().body("Account not found"),
    }
}

pub async fn get_block(data: web::Data<Mutex<Blockchain>>, path: web::Path<u64>) -> impl Responder {
    let blockchain = data.lock().unwrap();
    let index = path.into_inner();

    match blockchain.get_block(index) {
        Some(block) => HttpResponse::Ok().json(BlockResponse {
            index: block.index,
            previous_hash: block.previous_hash,
            timestamp: block.timestamp,
            nonce: block.nonce,
            miner: block.miner,
            signature: block.signature,
            reward: block.reward,
            transactions: block.transactions,
            hash: block.hash,
        }),
        None => HttpResponse::NotFound().body("Block not found"),
    }
}

pub async fn get_transaction_fee(data: web::Data<Mutex<Blockchain>>, req: web::Json<TransactionFeeRequest>) -> impl Responder {
    let blockchain = data.lock().unwrap();

    let transaction = Transaction::new(req.sender.clone(), req.receiver.clone(), req.amount, 1, String::new());
    let fee = blockchain.calculate_transaction_fee(&transaction);

    HttpResponse::Ok().json(TransactionFeeResponse { fee })
}

pub async fn get_blockchain_info(data: web::Data<Mutex<Blockchain>>) -> impl Responder {
    let blockchain = data.lock().unwrap();

    HttpResponse::Ok().json(BlockchainInfoResponse {
        current_height: blockchain.blocks.len() as u64,
        current_difficulty: blockchain.adjust_difficulty(),
        pending_transactions: blockchain.pending_transactions.len(),
        total_accounts: blockchain.accounts.len(),
    })
}

pub async fn get_pending_transactions(data: web::Data<Mutex<Blockchain>>) -> impl Responder {
    let blockchain = data.lock().unwrap();

    HttpResponse::Ok().json(PendingTransactionsResponse {
        transactions: blockchain.pending_transactions.iter().cloned().collect(),
    })
}

pub async fn submit_block(data: web::Data<Mutex<Blockchain>>, req: web::Json<SubmitBlockRequest>) -> impl Responder {
    let mut blockchain = data.lock().unwrap();

    let block = Block {
        index: req.index,
        previous_hash: req.previous_hash.clone(),
        timestamp: req.timestamp,
        nonce: req.nonce,
        miner: req.miner.clone(),
        signature: req.signature.clone(),
        reward: req.reward,
        transactions: req.transactions.clone(),
        hash: req.hash.clone(),
    };

    blockchain.db.store_block(&block);
    blockchain.blocks.push(block);

    HttpResponse::Ok().body("Block submitted")
}

pub async fn validate_chain(data: web::Data<Mutex<Blockchain>>) -> impl Responder {
    let blockchain = data.lock().unwrap();

    for (i, block) in blockchain.blocks.iter().enumerate() {
        if i > 0 {
            let previous_block = &blockchain.blocks[i - 1];
            if block.previous_hash != previous_block.hash {
                return HttpResponse::BadRequest().body(format!("Invalid chain at block {}", i));
            }
        }
    }

    HttpResponse::Ok().body("Blockchain is valid")
}

pub async fn get_account_history(data: web::Data<Mutex<Blockchain>>, path: web::Path<String>) -> impl Responder {
    let blockchain = data.lock().unwrap();
    let address = path.into_inner();
    let mut history = Vec::new();

    for block in &blockchain.blocks {
        for tx in &block.transactions {
            if tx.sender == address || tx.receiver == address {
                history.push(tx.clone());
            }
        }
    }

    HttpResponse::Ok().json(AccountHistoryResponse {
        transactions: history,
    })
}

