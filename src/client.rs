use enet::{Address, BandwidthLimit, ChannelLimit, Enet, Event, Packet, PacketMode};
use lipsum::lipsum;
use std::net::Ipv4Addr;

const CHANNEL_LIM: ChannelLimit = ChannelLimit::Maximum;
const TIMEOUT: u32 = 1_000;

pub struct ClientParams {
    pub ip: Ipv4Addr,
    pub port: u16,
    pub incoming_bandwidth: BandwidthLimit,
    pub outcoming_bandwidth: BandwidthLimit,
}

impl Default for ClientParams {
    fn default() -> Self {
        ClientParams {
            ip: Ipv4Addr::LOCALHOST,
            port: 9001,
            incoming_bandwidth: BandwidthLimit::Unlimited,
            outcoming_bandwidth: BandwidthLimit::Unlimited,
        }
    }
}

pub fn start_client(params: ClientParams) {
    let ClientParams {
        ip,
        port,
        incoming_bandwidth,
        outcoming_bandwidth,
    } = params;
    let enet = Enet::new().expect("could not start server");
    let mut host = enet
        .create_host::<()>(
            None,
            10,
            CHANNEL_LIM,
            incoming_bandwidth,
            outcoming_bandwidth,
        )
        .expect("could not create host");

    host.connect(&Address::new(ip, port), 10, 0)
        .expect("connect failed");

    let mut peer = loop {
        let e = host.service(TIMEOUT).expect("service failed");
        let e = match e {
            Some(ev) => ev,
            _ => continue,
        };
        println!("[client] event: {:#?}", e);
        match e {
            Event::Connect(ref p) => {
                break p.clone();
            }
            Event::Disconnect(ref p, r) => {
                println!("connection unsuccessful, peer: {:?}, reason: {}", p, r);
                std::process::exit(0);
            }
            Event::Receive { .. } => {
                panic!("unexpected Recieve-event while waiting for connection")
            }
        };
    };

    peer.send_packet(
        Packet::new(
            lipsum(25).into_bytes().as_slice(),
            PacketMode::ReliableSequenced,
        )
        .unwrap(),
        1,
    )
    .unwrap();

    peer.disconnect_later(5);

    loop {
        let e = host.service(1000).unwrap();
        println!("received event: {:#?}", e);
    }
}
