// https://github.com/libp2p/rust-libp2p/blob/master/transports/webrtc/examples/listen_ping.rs

use anyhow::Result;
use futures::StreamExt;
use libp2p::{
    core::muxing::StreamMuxerBox,
    identity,
    multiaddr::{Multiaddr, Protocol},
    ping,
    swarm::{keep_alive, NetworkBehaviour, Swarm, SwarmEvent},
    webrtc, Transport,
};
use rand::thread_rng;
use std::net::Ipv6Addr;
use void::Void;

/// An example WebRTC server that will accept connections and run the ping protocol on them.
#[tokio::main]
async fn main() -> Result<()> {
    // set up logging
    env_logger::init();

    let mut swarm = create_swarm()?;

    let address = Multiaddr::from(Ipv6Addr::UNSPECIFIED)
        .with(Protocol::Udp(42069))
        .with(Protocol::WebRTC);

    swarm.listen_on(address)?;

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                // check if address string contains "::" at all, if so skip the connection prompt
                if !address.to_string().contains("::") {
                    // add p2p PeerId to address as p2p Protocol
                    let full_address = address
                        .with(Protocol::P2p(*swarm.local_peer_id().as_ref()))
                        .to_string();

                    eprintln!("\nConnect with: \n{full_address}\n");
                }
            }
            event => eprintln!("🎇  Event: {event:?}\n"),
        }
    }
}

fn create_swarm() -> Result<Swarm<Behaviour>> {
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = id_keys.public().to_peer_id();
    let transport = webrtc::tokio::Transport::new(
        id_keys,
        webrtc::tokio::Certificate::generate(&mut thread_rng())?,
    );

    let transport = transport
        .map(|(peer_id, conn), _| (peer_id, StreamMuxerBox::new(conn)))
        .boxed();

    Ok(Swarm::with_tokio_executor(
        transport,
        Behaviour::default(),
        peer_id,
    ))
}

#[derive(NetworkBehaviour, Default)]
#[behaviour(
    out_event = "Event",
    event_process = false,
    prelude = "libp2p_swarm::derive_prelude"
)]
struct Behaviour {
    ping: ping::Behaviour,
    keep_alive: keep_alive::Behaviour,
}

#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
enum Event {
    Ping(ping::Event),
}

impl From<ping::Event> for Event {
    fn from(e: ping::Event) -> Self {
        Event::Ping(e)
    }
}

impl From<Void> for Event {
    fn from(event: Void) -> Self {
        void::unreachable(event)
    }
}
