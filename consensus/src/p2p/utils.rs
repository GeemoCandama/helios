use libp2p::core::{
    identity::Keypair, muxing::StreamMuxerBox, transport::Boxed, PeerId,
};
use libp2p::bandwidth::{BandwidthLogging, BandwidthSinks};
use libp2p::Transport;
use std::sync::Arc;
use std::time::Duration;

type BoxedTransport = Boxed<(PeerId, StreamMuxerBox)>;

// Supports TCP/IP, noise as the encryption protocol, and mplex as the multiplexing protocol
pub fn build_transport(
    local_private_key: Keypair,
) -> std::io::Result<(BoxedTransport, Arc<BandwidthSinks>)> {
    let tcp = libp2p::tcp::tokio::Transport::new(libp2p::tcp::Config::default().nodelay(true));
    let transport = libp2p::dns::TokioDnsConfig::system(tcp)?;

    let (transport, bandwidth) = BandwidthLogging::new(transport);

    // mplex config
    let mut mplex_config = libp2p::mplex::MplexConfig::new();
    mplex_config.set_max_buffer_size(256);
    mplex_config.set_max_buffer_behaviour(libp2p::mplex::MaxBufferBehaviour::Block);

    // yamux config
    let mut yamux_config = libp2p::yamux::YamuxConfig::default();
    yamux_config.set_window_update_mode(libp2p::yamux::WindowUpdateMode::on_read());

    // Authentication
    Ok((
        transport
            .upgrade(libp2p::core::upgrade::Version::V1)
            .authenticate(generate_noise_config(&local_private_key))
            .multiplex(libp2p::core::upgrade::SelectUpgrade::new(
                yamux_config,
                mplex_config,
            ))
            .timeout(Duration::from_secs(10))
            .boxed(),
        bandwidth,
    ))
}

// Generate authenticated XX Noise config from identity keys
fn generate_noise_config(
    identity_keypair: &Keypair,
) -> libp2p::noise::NoiseAuthenticated<libp2p::noise::XX, libp2p::noise::X25519Spec, ()> {
    let static_dh_keys = libp2p::noise::Keypair::<libp2p::noise::X25519Spec>::new()
        .into_authentic(identity_keypair)
        .expect("signing can only fail once during node initialization");
    libp2p::noise::NoiseConfig::xx(static_dh_keys).into_authenticated()
}
