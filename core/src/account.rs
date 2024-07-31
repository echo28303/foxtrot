// core/src/account.rs
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub address: String,
    pub balance: u64,
}

impl Account {
    pub fn new(address: String, balance: u64) -> Self {
        Account { address, balance }
    }
}

