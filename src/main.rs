// Modules
mod client;
mod server;

// External code
#[macro_use]
extern crate clap;
use clap::ArgMatches;
use serde_derive::{Deserialize, Serialize};
use std::{thread, time};

// Internal code
use client::{Client, ClientParams};
use server::{Server, ServerParams};

#[derive(Serialize, Deserialize, Debug)]
pub enum DataType {
    Coords { x: f32, y: f32, z: f32 },
    Text { string: String },
}

const SERVER_ADDRESS: &str = "127.0.0.1:9001";
const CLIENT_ADDRESS: &str = "127.0.0.1:9002";

fn main() {
    let yaml = load_yaml!("main.yml");
    let matches = clap::App::from_yaml(yaml)
        .version(crate_version!())
        .get_matches();
    let server_address = SERVER_ADDRESS.parse().unwrap();

    if let Some(matches) = matches.subcommand_matches("server") {
        // Server sub-command
        let headless = value_t!(matches, "headless", bool).unwrap();
        let max_players = value_t!(matches, "max_players", u8).unwrap();
        let mut server = Server::new(server_address);
        server.start(headless);
    } else if let Some(matches) = matches.subcommand_matches("client") {
        // Client sub-command
        let client_address = CLIENT_ADDRESS.parse().unwrap();
        let mut client = Client::new(server_address, client_address);
        client.start();
    }
}
