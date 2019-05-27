//! Client module

// External code
use bincode::serialize;
use crossbeam_channel::{Receiver, Sender};
use laminar::{ErrorKind, Packet, Socket, SocketEvent};
use std::net::{Ipv4Addr, SocketAddr};
use std::thread;

// Internal code
use crate::DataType;

pub struct ClientParams {}

pub struct Client {
    server_address: SocketAddr,
    client_address: SocketAddr,
    packet_sender: Sender<Packet>,
    event_receiver: Receiver<SocketEvent>,
    polling_thread: thread::JoinHandle<Result<(), ErrorKind>>,
}

impl Client {
    pub fn new(server_address: SocketAddr, client_address: SocketAddr) -> Self {
        let (mut socket, packet_sender, event_receiver) = Socket::bind(client_address).unwrap();
        let polling_thread = thread::spawn(move || socket.start_polling());
        Client {
            server_address,
            client_address,
            packet_sender,
            event_receiver,
            polling_thread,
        }
    }

    pub fn start(&mut self) -> bool {
//        let stdin = stdin();
//        let mut buffer = String::new();
//        loop {
//            buffer.clear();
//            stdin.read_line(&mut buffer);
//            let line = buffer.replace(|x| x == '\n' || x == '\r', "");
//            if line == "q" {
//                break;
//            }
//            client.send(DataType::Text {
//                string: String::from("Hello world!"),
//            });
//        }
        false
    }

    pub fn send(&mut self, data: DataType) {
        println!("Attempting to send {:?} to server {}", data, self.server_address);
        let serialized = serialize(&data);
        match serialized {
            Ok(raw) => {
                self.packet_sender
                    .send(Packet::reliable_unordered(self.server_address, raw.clone()))
                    .unwrap();
                println!("Sent packet {:?} to server {}", raw, self.server_address);
            }
            Err(e) => {
                println!("Error in serializing: {:?}", e);
            }
        }
    }
}
