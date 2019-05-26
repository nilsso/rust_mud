use enet::{Address, BandwidthLimit, ChannelLimit, Enet, Event};
use std::net::Ipv4Addr;

const CHANNEL_LIM: ChannelLimit = ChannelLimit::Maximum;
const TIMEOUT: u32 = 1_000;

pub struct ServerParams {
    pub max_players: u32,
    pub ip: Ipv4Addr,
    pub port: u16,
    pub incoming_bandwidth: BandwidthLimit,
    pub outcoming_bandwidth: BandwidthLimit,
}

impl Default for ServerParams {
    fn default() -> Self {
        ServerParams {
            max_players: 8,
            ip: Ipv4Addr::LOCALHOST,
            port: 9001,
            incoming_bandwidth: BandwidthLimit::Unlimited,
            outcoming_bandwidth: BandwidthLimit::Unlimited,
        }
    }
}

pub fn start_server(params: ServerParams) {
    let ServerParams {
        max_players,
        ip,
        port,
        incoming_bandwidth,
        outcoming_bandwidth,
    } = params;
    let enet = Enet::new().expect("could not start server");
    let local_addr = Address::new(ip, port);
    let mut host = enet
        .create_host::<()>(
            Some(&local_addr),
            10,
            CHANNEL_LIM,
            incoming_bandwidth,
            outcoming_bandwidth,
        )
        .expect("could not create host");
    loop {
        let e = host.service(TIMEOUT).expect("service failed");
        println!("received event: {:#?}", e);
        match e {
            Some(Event::Connect(_)) => println!("new connection!"),
            Some(Event::Disconnect(..)) => println!("disconnect!"),
            Some(Event::Receive {
                channel_id,
                ref packet,
                ..
            }) => println!(
                "got packet on channel {}, content: '{}'",
                channel_id,
                std::str::from_utf8(packet.data()).unwrap()
            ),
            _ => {}
        }
    }
}
