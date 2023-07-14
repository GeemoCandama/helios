use std::sync::{Mutex, Arc};

use eyre::Result;
use libp2p::PeerId;
use libp2p::swarm::{SwarmBuilder, ConnectionLimits};
use crate::types::{BeaconBlock, Bootstrap, FinalityUpdate, OptimisticUpdate, Update};
use async_trait::async_trait;
use libp2p::{
    swarm::NetworkBehaviour,
    identity::Keypair,
};
use log::{debug, info};


use crate::p2p::discovery::Discovery;
use crate::rpc::ConsensusNetworkInterface;

use super::build_transport;

pub struct P2pNetwork {
    swarm: Arc<Mutex<libp2p::swarm::Swarm<Behaviour>>>,
}

impl P2pNetwork {
    // Starts the network
    pub async fn start(&mut self) -> Result<()> {
        let mut swarm = self.swarm.lock().unwrap();
        info!("Starting Libp2p network");
        debug!("Attempting to open listening ports");
        swarm.listen_on("/ip4/0.0.0.0/tcp/9000".parse()?)?;

        Ok(())
    }
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl ConsensusNetworkInterface for P2pNetwork {
    // TODO: Really implement this function
    async fn new(_path: &str) -> Self {
        let local_key = Keypair::generate_secp256k1();
        let local_peer_id = PeerId::from(local_key.public());
        let config = super::config::Config::default();
        let mut discovery = Discovery::new(&local_key, config).await.unwrap();
        discovery.find_peers();
        let (transport, bandwidth) = build_transport(local_key.clone())
            .expect("Failed to build transport");
        
        let behaviour = Behaviour {
            discovery,
        };

        let mut swarm = SwarmBuilder::with_tokio_executor(transport, behaviour, local_peer_id)
            .notify_handler_buffer_size(std::num::NonZeroUsize::new(7).expect("not zero"))
            .connection_event_buffer_size(64)
            .connection_limits(
                ConnectionLimits::default()
                    .with_max_pending_incoming(Some(5))
                    .with_max_pending_outgoing(Some(16))
                    .with_max_established_per_peer(Some(1)),
            )
            .build();

        P2pNetwork {
            swarm: Arc::new(Mutex::new(swarm)),
        }
    }

    async fn get_bootstrap(&self, _block_root: &'_ [u8]) -> Result<Bootstrap> {
        unimplemented!()
    }

    async fn get_updates(&self, _period: u64, _count: u8) -> Result<Vec<Update>> {
        unimplemented!()
    }

    async fn get_finality_update(&self) -> Result<FinalityUpdate> {
        unimplemented!()
    }

    async fn get_optimistic_update(&self) -> Result<OptimisticUpdate> {
        unimplemented!()
    }

    async fn get_block(&self, _slot: u64) -> Result<BeaconBlock> {
        unimplemented!()
    }

    async fn chain_id(&self) -> Result<u64> {
        Ok(1)
    }
}

#[derive(NetworkBehaviour)]
struct Behaviour {
    discovery: Discovery,
}
