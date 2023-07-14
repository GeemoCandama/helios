#[cfg(feature = "consensus_p2p")]
use std::path::PathBuf;

use env_logger::Env;
use eyre::Result;
use helios::{config::networks::Network, prelude::*};

use consensus::p2p::P2pNetworkInterface;
use consensus::p2p::Discovery;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let untrusted_rpc_url = "https://eth-mainnet.g.alchemy.com/v2/nObEU8Wh4FIT-X_UDFpK9oVGiTHzznML";
    log::info!("Using untrusted RPC URL [REDACTED]");

    Discovery::new()

    //let mut client: Client<FileDB, P2pNetworkInterface> = ClientBuilder::new()
    //    .network(Network::MAINNET)
    //    .consensus_rpc("")
    //    .execution_rpc(untrusted_rpc_url)
    //    .load_external_fallback()
    //    .data_dir(PathBuf::from("/tmp/helios"))
    //    .build()
    //    .await?;

    // client.start().await?;

    Ok(())
}
