mod client;
mod server;

#[macro_use]
extern crate clap;

use clap::ArgMatches;

use client::*;
use server::*;

fn main() {
    let yaml = load_yaml!("main.yml");
    let matches = clap::App::from_yaml(yaml).get_matches();
    if let Some(matches) = matches.subcommand_matches("server") {
        println!("starting server...");
        start_server(ServerParams {
            max_players: value_t!(matches, "max_players", u32).unwrap_or_else(|e| e.exit()),
            ..Default::default()
        });
    } else if let Some(matches) = matches.subcommand_matches("client") {
        println!("starting client...");
        //start_client(ClientParams {..});
        start_client(ClientParams {
            ..Default::default()
        });
    }
}
