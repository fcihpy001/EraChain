use std::sync::Arc;
use crate::chain::block::Block;
use crate::chain::consensus::ConsensusType;

pub struct BlockChain {
    name: String,
    blocks: Vec<Block>,
    storage: Arc<T>,
    consensusType: ConsensusType
}