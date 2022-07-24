use crate::network::command::{Message, MessageType};
use anyhow::Result;
use futures::future::err;
use libp2p::{
    gossipsub::{Gossipsub, GossipsubConfig, GossipsubEvent, MessageAuthenticity},
    identity::Keypair,
    mdns::{Mdns, MdnsEvent},
    swarm::NetworkBehaviourEventProcess,
    NetworkBehaviour,
};
use tokio::sync::mpsc;
use tracing::{error, info};

#[derive(NetworkBehaviour)]
#[behaviour(event_process = true)]
pub struct BlockchainBehaviour {
    pub gossipsub: Gossipsub,
    pub mdns: Mdns,
    pub msg_sender: mpsc::UnboundedSender<Message>,
}
impl BlockchainBehaviour {
    pub async fn new(
        key_pair: Keypair,
        config: GossipsubConfig,
        msg_sender: mpsc::UnboundedSender<MessageType>,
    ) -> Result<Self> {
        Ok(Self {
            gossipsub: Gossipsub::new(MessageAuthenticity::Signed(key_pair), config).unwrap(),
            mdns: Mdns::new(Default::default()).await?,
            msg_sender,
        })
    }
}

impl NetworkBehaviourEventProcess<GossipsubEvent> for BlockchainBehaviour {
    fn inject_event(&mut self, event: GossipsubEvent) {
        match event {
            GossipsubEvent::Message {
                propagation_source: p_id,
                message_id: m_id,
                message,
            } => {
                let msg: Message = serde_json::from_slice(&message.data).unwrap();
                info!(
                    "Recive message: {:?} with id: {} from peer: {}",
                    msg, m_id, pid
                );
                if let Err(e) = self.msg_sender.send(msg) {
                    error!("error sending messages via channel {}", e);
                }
            }
            GossipsubEvent::Subscribed { peer_id, topic } => {
                info!("Subscribe topic {}  from Id {}", topic, peer_id);
            }
            GossipsubEvent::Unsubscribed { peer_id, topic } => {
                info!()
            }
            _ => {}
        }
    }
}

impl NetworkBehaviourEventProcess<MdnsEvent> for BlockchainBehaviour {
    fn inject_event(&mut self, event: MdnsEvent) {
        match event {
            MdnsEvent::Discovered(list) => {
                for (peer_id, address) in list {
                    println!("发现新的节点：{} with addr {}", &peer_id, &address);
                    self.gossipsub.add_explicit_peer(&peer_id);
                }
            }
            MdnsEvent::Expired(list) => {
                for (peer_id, address) in list {
                    println!("有节点离开网络了{} with addr {}", &peer_id, &address);
                    self.gossipsub.remove_explicit_peer(&peer_id);
                }
            }
        }
    }
}
