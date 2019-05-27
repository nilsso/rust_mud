// Modules
mod client;
mod server;

// External code
#[macro_use]
extern crate clap;
use clap::ArgMatches;
use serde_derive::{Deserialize, Serialize};

// Internal code
use client::{Client, ClientParams};
use server::{Server, ServerParams};

#[derive(Serialize, Deserialize)]
pub enum DataType {
    Coords { x: f32, y: f32, z: f32 },
    Text { string: String },
}

//let address = "127.0.0.1:9001".parse().unwrap();
//let server = Server::new(address);

fn main() {
    let yaml = load_yaml!("main.yml");
    let matches = clap::App::from_yaml(yaml).get_matches();
    if let Some(matches) = matches.subcommand_matches("server") {
    } else if let Some(matches) = matches.subcommand_matches("client") {
    }
}
