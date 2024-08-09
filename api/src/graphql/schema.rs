use async_graphql::{Context, EmptyMutation, Object, Schema, SimpleObject, Subscription};
use async_graphql::futures_util::Stream;
use async_graphql::futures_util::stream::StreamExt;
use std::sync::Mutex;
use std::time::Duration;
use tokio::time::interval;
use core::blockchain::Blockchain;
use core::transaction::Transaction;
use core::block::Block;

pub type MySchema = Schema<QueryRoot, EmptyMutation, SubscriptionRoot>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn get_account(&self, ctx: &Context<'_>, address: String) -> Option<Account> {
        let blockchain = ctx.data::<Mutex<Blockchain>>().unwrap().lock().unwrap();
        blockchain.get_account(&address).map(|account| Account {
            address: account.address.clone(),
            balance: account.balance,
        })
    }

    async fn get_block(&self, ctx: &Context<'_>, index: u64) -> Option<BlockData> {
        let blockchain = ctx.data::<Mutex<Blockchain>>().unwrap().lock().unwrap();
        blockchain.get_block(index).map(|block| BlockData {
            index: block.index,
            previous_hash: block.previous_hash.clone(),
            timestamp: block.timestamp,
            nonce: block.nonce,
            miner: block.miner.clone(),
            signature: block.signature.clone(),
            reward: block.reward,
            transactions: block.transactions.clone(),
            hash: block.hash.clone(),
        })
    }

    async fn get_blockchain_info(&self, ctx: &Context<'_>) -> BlockchainInfo {
        let blockchain = ctx.data::<Mutex<Blockchain>>().unwrap().lock().unwrap();
        BlockchainInfo {
            current_height: blockchain.blocks.len() as u64,
            current_difficulty: blockchain.adjust_difficulty(),
            pending_transactions: blockchain.pending_transactions.len(),
            total_accounts: blockchain.accounts.len(),
        }
    }

    async fn get_pending_transactions(&self, ctx: &Context<'_>) -> Vec<Transaction> {
        let blockchain = ctx.data::<Mutex<Blockchain>>().unwrap().lock().unwrap();
        blockchain.pending_transactions.clone()
    }
}

pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn new_block_notification(&self, ctx: &Context<'_>) -> impl Stream<Item = BlockData> {
        let blockchain = ctx.data::<Mutex<Blockchain>>().unwrap();
        let mut interval = interval(Duration::from_secs(1));

        async_stream::stream! {
            loop {
                interval.tick().await;
                let block = blockchain.lock().unwrap().blocks.last().cloned();
                if let Some(block) = block {
                    yield BlockData {
                        index: block.index,
                        previous_hash: block.previous_hash.clone(),
                        timestamp: block.timestamp,
                        nonce: block.nonce,
                        miner: block.miner.clone(),
                        signature: block.signature.clone(),
                        reward: block.reward,
                        transactions: block.transactions.clone(),
                        hash: block.hash.clone(),
                    };
                }
            }
        }
    }
}

#[derive(SimpleObject)]
struct Account {
    address: String,
    balance: u64,
}

#[derive(SimpleObject)]
struct BlockData {
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

#[derive(SimpleObject)]
struct BlockchainInfo {
    current_height: u64,
    current_difficulty: u64,
    pending_transactions: usize,
    total_accounts: usize,
}
