//! Client module

// External code
use bincode::serialize;
use crossbeam_channel::{Receiver, Sender};
use getset::Getters;
use laminar::{ErrorKind, Packet, Socket, SocketEvent};
use rustyline;
use std::net::SocketAddr;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// Internal code
use crate::server::NetMessage;

/// Client struct
#[derive(Getters)]
#[get = "pub"]
pub struct Client {
    server_address: SocketAddr,
    client_address: SocketAddr,
    packet_sender: Sender<Packet>,
    event_receiver: Receiver<SocketEvent>,
    polling_thread: thread::JoinHandle<Result<(), ErrorKind>>,
}

/// Message type for between main thread and REPL thread
#[allow(dead_code)]
enum InnerMessage {
    Confirm,
    Shutdown,
    NetMessage(NetMessage),
}

impl Client {
    pub fn new(server_address: SocketAddr, client_address: SocketAddr) -> Self {
        let (mut socket, packet_sender, event_receiver) = Socket::bind(server_address).unwrap();
        let polling_thread = thread::spawn(move || socket.start_polling());
        Self {
            server_address,
            client_address,
            packet_sender,
            event_receiver,
            polling_thread,
        }
    }

    pub fn start(&mut self, poll: Duration) -> bool {
        let (event_receiver_tx, event_receiver_rx) = mpsc::channel();
        let (_repl_tx, repl_rx) = mpsc::channel();
        let _repl_thread = thread::spawn(move || {
            Client::repl(
                event_receiver_tx,
                repl_rx);
        });
        loop {
            match self.event_receiver.recv_timeout(poll) {
                Ok(SocketEvent::Packet(packet)) => {
                    self.receive_action(packet);
                }
                Ok(SocketEvent::Connect(_)) => {
                    // Client connected
                }
                Ok(SocketEvent::Timeout(_)) => {
                    // Client timed out
                }
                Err(_) => {
                    // Polling timed out
                }
            }
            match event_receiver_rx.try_recv() {
                Ok(InnerMessage::Shutdown) => break,
                Ok(InnerMessage::NetMessage(data)) => {
//                    self.packet_sender
//                        .send(Packet::reliable_unordered(self.server_address, data));
                },
                Ok(_) => {},
                Err(_) => {}
            }
        }
        #[allow(unreachable_code)]
        true
    }

    /// Receive message from server
    ///
    /// Receives `ServerSendMessage`
    #[allow(dead_code)]
    fn receive_action(&mut self, _packet: Packet) {}

    /// Send message to server
    ///
    /// Sends `ServerReceiveMessage`
    #[allow(dead_code)]
    fn send_action(&mut self, data: NetMessage) {
        match serialize(&data) {
            Ok(data) => {
                println!("Sending data {:?} to server {}", data, self.server_address);
                self.packet_sender
                    .send(Packet::reliable_unordered(self.server_address, data))
                    .unwrap();
            }
            Err(e) => {
                println!("Error in serializing: {:?}", e);
            }
        }
    }

    fn repl(
        event_receiver_tx: mpsc::Sender<InnerMessage>,
        _repl_rx: mpsc::Receiver<InnerMessage>,
    ) {
        let mut rl = rustyline::Editor::<()>::new();
        rl.load_history("client.txt").ok();
        loop {
            if let Ok(line) = rl.readline("> ") {
                let line = line.replace(|c| c == '\n' || c == '\r', "");
                let args: Vec<&str> = line.split_whitespace().collect();
                if !args.is_empty() {
                    match args[0] {
                        "quit" => break,
                        "help" => println!("{}", REPL_HELP),
                        "tellserver" => {

                        },
                        _ => {
                            println!("Unknown command. Type 'help' for usage")
                        },
                    }
                    rl.add_history_entry(line.as_str());
                }
            }
        }
        if rl.save_history("client.txt").is_err() {
            println!("Error, could not save command history");
        }
        event_receiver_tx.send(InnerMessage::Shutdown).ok();
    }
}

const REPL_HELP: &str = "\
COMMANDS
    quit : Shutdown client
    help : Print this message";
