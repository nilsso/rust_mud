//! Game client module

use std::net::Ipv4Addr;
use laminar::{Socket, Packet};

const TIMEOUT: u32 = 1_000;

pub struct ClientParams {
    pub ip: Ipv4Addr,
    pub port: u16,
}

impl Default for ClientParams {
    fn default() -> Self {
        ClientParams {
            ip: Ipv4Addr::LOCALHOST,
            port: 9001
        }
    }
}

pub fn start_client(params: ClientParams) {
}
