//! Server module

// External code
use bincode::deserialize;
use crossbeam_channel::{Receiver, Sender};
use laminar::{ErrorKind, Packet, Socket, SocketEvent};
use std::net::{Ipv4Addr, SocketAddr};
use std::thread;
use std::time::Duration;

// Internal code
use crate::DataType;
mod server_repl;

pub struct ServerParams {
    max_players: u8,
}

pub struct Server {
    address: SocketAddr,
    packet_sender: Sender<Packet>,
    event_receiver: Receiver<SocketEvent>,
    polling_thread: thread::JoinHandle<Result<(), ErrorKind>>,
}

impl Server {
    pub fn new(address: SocketAddr) -> Self {
        let (mut socket, packet_sender, event_receiver) = Socket::bind(address).unwrap();
        let polling_thread = thread::spawn(move || socket.start_polling());
        Server {
            address,
            packet_sender,
            event_receiver,
            polling_thread,
        }
    }

    pub fn address(&self) -> SocketAddr {
        self.address
    }

    pub fn start(&mut self, headless: bool) -> bool {
        if !headless {
            if let Err(e) = server_repl::server_repl() {
                eprintln!("Server quit unexpectedly! Error: {}", e);
            }
        }
        false
    }

    pub fn receive(&mut self) {
        println!("Receive...");
        let result = self.event_receiver.recv();
        match result {
            Ok(SocketEvent::Packet(packet)) => {
                println!("{} packet!", packet.addr());
                let raw: &[u8] = packet.payload();
                let deserialized: DataType = deserialize(&raw).unwrap();
                self.perform_action(deserialized);
            }
            Ok(SocketEvent::Timeout(address)) => {
                println!("{} timeout!", address);
            }
            Ok(SocketEvent::Connect(address)) => {
                println!("{} connected!", address);
            }
            Err(e) => {
                println!("Error in receiving: {:?}", e);
            }
        }
    }

    pub fn perform_action(&mut self, data: DataType) {
        match data {
            DataType::Coords { x, y, z } => {
                println!("Received coordinate: x={} y={} z={}", x, y, z);
            }
            DataType::Text { string } => {
                println!("Received text: '{}'", string);
            }
        }
    }
}
