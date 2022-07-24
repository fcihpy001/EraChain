use crate::chain::blockChain::BlockChain;

struct Node {
    id: String,
    chain: BlockChain,
}

mod behaviour;
mod command;
mod message;
mod node;
use anyhow::Result;
use libp2p::{
    core::upgrade,
    gossipsub::{
        GossipsubConfigBuilder, GossipsubMessage, IdentTopic as Topic, MessageId, ValidationMode,
    },
    identity::Keypair,
    noise,
    swarm::SwarmBuilder,
    tcp::TokioTcpConfig,
    yamux, PeerId, Swarm, Transport,
};
use once_cell::sync::Lazy;
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
    sync::Arc,
    time::Duration,
};
use tokio::sync::{mpsc, Mutex};

static ID_KEYS: Lazy<Keypair> = Lazy::new(Keypair::generate_ed25519);
static PEER_ID: Lazy<PeerId> = Lazy::new(|| PeerId::from(ID_KEYS.public()));
static BLOCK_TOPIC: Lazy<Topic> = Lazy::new(|| Topic::new("blocks"));
static TRANX_TOPIC: Lazy<Topic> = Lazy::new(|| Topic::new("tranxs"));
static WALLET_MAP: Lazy<Arc<Mutex<HashMap<String, String>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));
