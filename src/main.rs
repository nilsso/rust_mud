//! Game executable

// Modules
pub mod client;
pub mod server;

// External code
use clap::{crate_version, load_yaml, value_t};
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;

// Internal code
use client::Client;
use server::Server;

const DEFAULT_SERVER_POLL_RATE: Duration = Duration::from_millis(1000);
const DEFAULT_CLIENT_POLL_RATE: Duration = Duration::from_millis(1000);

fn main() {
    let yaml = load_yaml!("main.yml");
    let matches = clap::App::from_yaml(yaml)
        .version(crate_version!())
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("server") {
        // Server sub-command

        // Clap arguments
        let server_ip = value_t!(matches, "ip", String).unwrap();
        let server_port = value_t!(matches, "port", String).unwrap();
        let server_address = format!("{}:{}", server_ip, server_port);
        let server_address: SocketAddr = SocketAddr::from_str(server_address.as_str()).unwrap();

        let max_players = value_t!(matches, "max_players", u8).unwrap();

        // Start
        let mut server = Server::new(server_address, max_players);
        server.start(DEFAULT_SERVER_POLL_RATE);
    } else if let Some(matches) = matches.subcommand_matches("client") {
        // Client sub-command

        // Clap arguments
        let server_ip = value_t!(matches, "ip", String).unwrap();
        let server_port = value_t!(matches, "port", String).unwrap();
        let server_address = format!("{}:{}", server_ip, server_port);
        let server_address: SocketAddr = SocketAddr::from_str(server_address.as_str()).unwrap();

        let client_ip = value_t!(matches, "ip", String).unwrap();
        let client_port = value_t!(matches, "port", String).unwrap();
        let client_address = format!("{}:{}", client_ip, client_port);
        let client_address = SocketAddr::from_str(client_address.as_str()).unwrap();

        // Start
        let mut client = Client::new(server_address, client_address);
        client.start(DEFAULT_CLIENT_POLL_RATE);
    }
}
