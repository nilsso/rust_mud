//! Game server module

use std::net::Ipv4Addr;
use laminar::{Socket, Packet};

const TIMEOUT: u32 = 1_000;

pub struct ServerParams {
    pub max_players: u32,
    pub ip: Ipv4Addr,
    pub port: u16,
}

impl Default for ServerParams {
    fn default() -> Self {
        ServerParams {
            max_players: 8,
            ip: Ipv4Addr::LOCALHOST,
            port: 9001,
        }
    }
}

pub fn start_server(params: ServerParams) {
}
