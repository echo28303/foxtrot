use crate::block::Block;
use rocksdb::{DB, Options, Error};

#[derive(Debug)]
pub struct Database {
    db: DB,
}

impl Database {
    pub fn new(path: &str) -> Self {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        let db = DB::open(&opts, path).expect("Failed to open database");
        Self { db }
    }

    pub fn store_block(&self, block: &Block) -> Result<(), Error> {
        let key = block.index.to_be_bytes();
        let value = bincode::serialize(block).expect("Failed to serialize block");
        self.db.put(key, value)?;
        Ok(())
    }

    pub fn get_block(&self, index: u64) -> Option<Block> {
        let key = index.to_be_bytes();
        match self.db.get(key) {
            Ok(Some(value)) => Some(bincode::deserialize(&value).expect("Failed to deserialize block")),
            Ok(None) => None,
            Err(_) => None,
        }
    }
}

