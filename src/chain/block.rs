use crate::chain::transaction::Transaction;



pub struct BlockHeader {
    version: String,
    timestamp: i64,
    prev_hash: String,
    hash: String,
    miner: String
}

pub struct Block {
    header: BlockHeader,
    transactions: Vec<Transaction>
}
