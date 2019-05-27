//! Client module

// External code
use std::thread;
use crossbeam_channel::{Receiver, Sender};
use laminar::{ErrorKind, Packet, Socket, SocketEvent};
use std::net::{Ipv4Addr, SocketAddr};

// Internal code
use crate::DataType;

pub struct ClientParams {

}

pub struct Client {
    packet_sender: Sender<Packet>,
    event_receiver: Receiver<SocketEvent>,
    polling_thread: thread::JoinHandle<Result<(), ErrorKind>>,
}

impl Client {
    pub fn new(address: SocketAddr) -> Self {
        let (mut socket, packet_sender, event_receiver) = Socket::bind(address).unwrap();
        let polling_thread = thread::spawn(move || socket.start_polling());
        Client {
            packet_sender,
            event_receiver,
            polling_thread,
        }
    }
}