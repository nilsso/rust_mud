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
        let params = ServerParams {
            max_players: value_t!(matches, "max_players", u32).unwrap_or_else(|e| e.exit()),
            ..Default::default()
        };

        start_server(params);
    } else if let Some(matches) = matches.subcommand_matches("client") {
        let params = ClientParams {..Default::default()};

        start_client(params);
    }
}
