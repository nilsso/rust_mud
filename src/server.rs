//! Server module

// External code
use std::thread;
use crossbeam_channel::{Receiver, Sender};
use laminar::{ErrorKind, Packet, Socket, SocketEvent};
use std::net::{Ipv4Addr, SocketAddr};
use bincode::deserialize;

// Internal code
use crate::DataType;

pub struct ServerParams {
    max_players: u8,
}

pub struct Server {
    packet_sender: Sender<Packet>,
    event_receiver: Receiver<SocketEvent>,
    polling_thread: thread::JoinHandle<Result<(), ErrorKind>>,
}

impl Server {
    pub fn new(address: SocketAddr) -> Self {
        let (mut socket, packet_sender, event_receiver) = Socket::bind(address).unwrap();
        let polling_thread = thread::spawn(move || socket.start_polling());
        Server {
            packet_sender,
            event_receiver,
            polling_thread,
        }
    }

    pub fn receive(&mut self) {
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

    pub fn perform_action(&mut self, data_type: DataType) {
        match data_type {
            DataType::Coords { x, y, z } => {
                println!("Received coordinate: x={} y={} z={}", x, y, z);
            }
            DataType::Text { string } => {
                println!("Received text: '{}'", string);
            }
        }
    }
}