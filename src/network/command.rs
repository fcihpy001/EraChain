use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Commands {
    Genesis(String),
    Blocks(String),
    Sync(String),
    CreateWallet(String),
    GetAddress(String),
    Trans {
        from: String,
        to: String,
        amount: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MessageType {
    Node,
    Chain {
        blocks: Vec<Block>,
        height: usize,
        to_addr: String,
    },
    Block {
        block: Block,
    },
    Version {
        best_height: usize,
        from_addr: String,
    },
}
