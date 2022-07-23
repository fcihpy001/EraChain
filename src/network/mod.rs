use crate::chain::blockChain::BlockChain;

struct Node {
    id: String,
    chain: BlockChain,
}

pub enum Message {
    Node,
    Chain,
    Block,
    Version
}

mod command;
mod behaviour;
mod message;
mod node;