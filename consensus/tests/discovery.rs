//#[cfg(feature = "p2p")]
use consensus::p2p::P2pNetwork;
use consensus::rpc::ConsensusNetworkInterface;

#[tokio::test]
async fn test_p2p_startup() {
    let mut network = P2pNetwork::new("test").await;
    network.start().await.expect("Failed to start p2p network");
}
