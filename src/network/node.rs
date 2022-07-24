use crate::network::behaviour::BlockchainBehaviour;
use crate::network::command::MessageType;
use anyhow::Result;
use libp2p::gossipsub::GossipsubEvent::Message;
use libp2p::gossipsub::IdentTopic as Topic;
use libp2p::identify::Identify;
use libp2p::swarm::SwarmEvent;
use libp2p::tcp::{GenTcpConfig, GenTcpTransport};
use libp2p::{
    core::upgrade,
    gossipsub::{GossipsubConfigBuilder, GossipsubMessage, MessageId, ValidationMode},
    identity, noise,
    swarm::SwarmBuilder,
    tcp::TokioTcpConfig,
    yamux, PeerId, Swarm, Transport,
};
use once_cell::sync::Lazy;
use std::io::{stdin, BufRead, BufReader};
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
    sync::Arc,
    time::Duration,
};
use tokio::sync::{mpsc, Mutex};
pub struct Node {
    swarm: Swarm<BlockchainBehaviour>,
    msg_reciver: mpsc::UnboundedReceiver<MessageType>,
}

impl Node {
    pub fn new() -> Result<Self> {
        let (tx, msg_receiver) = mpsc::unbounded_channel();
        let topic = Topic::new("block");
        Ok(Self {
            swarm: create_swarm(vec![topic], tx).await?,
            msg_reciver,
        })
    }

    pub fn start(&mut self) -> Result<()> {
        let mut stdin = BufReader::new(stdin()).lines();
        loop {
            tokio::select! {
                line = stdin.next_line() => {

                 },
                 message = self.msg_reciver.recv() =>  messages{
                    if let Some(msg) = messages {
                        match msg {
                            Messages:Version{},
                            Messages:Blocks{},
                            Messages::Chain{}
                        }}
                 },
                event = self.swarm.select_next_some() => {
                    if let SwarmEvent::NewListenAddr {address, .. } => {
                        println!("Listening on: {:?}", address);
                    }
                }
            }
        }
    }
}

fn create_swarm(
    topics: Vec<Topic>,
    msg_sender: mpsc::UnboundedSender<MessageType>,
) -> Result<Swarm<BlockchainBehaviour>> {
    //create peer_id
    let key_pair = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(key_pair.public());
    //config transport
    let nosie_keys = noise::Keypair::<noise::X25519Spec>::new().into_authentication(&key_pair)?;
    let transport = TokioTcpConfig::new()
        .nodely(true)
        .upgrade(upgrade::Version::V1)
        .authenticate(noise::NoiseConfig::xx(nosie_keys).into_authenticated())
        .multiplex(yamux::YamuxConfig::default())
        .boxed();
    //config behaviour
    let mut behaviour = config_behaviour(msg_sender).unwrap();

    for topic in topics.iter() {
        behaviour.gossipsub.subscribe(topic).unwrap();
    }

    let swarm = SwarmBuilder::new(transport, behaviour, peer_id)
        .executor(Box::new(|fut| tokio::spawn(fut)))
        .build();
    Ok(swarm)
}

fn config_behaviour(msg_sender: mpsc::UnboundedSender<MessageType>) -> Result<BlockchainBehaviour> {
    //create block
    let message_id_fn = |message: &GossipsubMessage| {
        let mut hasher = DefaultHasher::new();
        message.data.hash(&mut hasher);
        MessageId::from(hasher.finish().to_string())
    };
    //create gossipsub-config
    let gossipsub_config = GossipsubConfigBuilder::default()
        .heartbeat_interval(Duration::from_secs(10))
        .validation_mode(ValidationMode::Strict)
        .message_id_fn(message_id_fn)
        .build()
        .expect("Valid config");

    let mut behaviour =
        BlockchainBehaviour::new(key_pair.clone(), gossipsub_config, msg_sender).await?;
    Ok(behaviour)
}

pub fn start() -> Result<()> {}
