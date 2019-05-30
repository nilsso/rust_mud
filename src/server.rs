//! Server module
#![allow(dead_code)]

// External code
use bincode::{serialize, deserialize};
use crossbeam_channel::{Receiver, Sender};
use getset::Getters;
use laminar::{ErrorKind, Packet, Socket, SocketEvent};
use serde_derive::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub enum NetMessage {
    Message {
        from: SocketAddr,
        message: String
    },
}

/// Server struct
#[derive(Getters)]
#[get = "pub"]
pub struct Server {
    address: SocketAddr,
    max_players: u8,
    packet_sender: Sender<Packet>,
    event_receiver: Receiver<SocketEvent>,
    polling_thread: thread::JoinHandle<Result<(), ErrorKind>>,
}

/// Message type for between main thread and REPL thread
#[derive(PartialEq, Debug)]
enum InnerMessage {
    Confirm,
    Shutdown,
    Send(NetMessage),
    Poll(Option<Duration>),
}

impl Server {
    pub fn new(address: SocketAddr, max_players: u8) -> Self {
        let (mut socket, packet_sender, event_receiver) = Socket::bind(address).unwrap();
        let polling_thread = thread::spawn(move || socket.start_polling());
        Self {
            address,
            max_players,
            packet_sender,
            event_receiver,
            polling_thread,
        }
    }

    pub fn start(&mut self, mut poll: Duration) -> bool {
        let (event_receiver_tx, event_receiver_rx) = mpsc::channel();
        let (repl_tx, repl_rx) = mpsc::channel();
        let _repl_thread = thread::spawn(move || {
            Server::repl(event_receiver_tx, repl_rx);
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
                Ok(InnerMessage::Poll(Some(p))) => {
                    poll = p;
                    repl_tx.send(InnerMessage::Confirm).ok();
                }
                Ok(InnerMessage::Poll(None)) => {
                    repl_tx.send(InnerMessage::Poll(Some(poll))).ok();
                }
                Ok(_) => {}
                Err(_) => {}
            }
        }
        true
    }

    /// Send message from client
    ///
    /// Sends `ServerReceiveMessage`
    fn receive_action(&mut self, packet: Packet) {
        let data: NetMessage = deserialize(packet.payload()).unwrap();
        println!("{:?}", data);
    }

    /// Send message to client
    ///
    /// Sends `ServerSendMessage`
    fn send_action(&mut self) {}

    fn repl(
        event_receiver_tx: mpsc::Sender<InnerMessage>,
        repl_rx: mpsc::Receiver<InnerMessage>,
    ) {
        let mut rl = rustyline::Editor::<()>::new();
        rl.load_history("server.txt").ok();
        loop {
            if let Ok(line) = rl.readline("> ") {
                let line = line.replace(|c| c == '\n' || c == '\r', "");
                let args: Vec<&str> = line.split_whitespace().collect();
                if !args.is_empty() {
                    match args[0] {
                        "quit" => break,
                        "help" => println!("{}", REPL_HELP),
                        "poll" => {
                            // TODO: Begs for a message register system
                            if args.len() < 2 {
                                // Request current polling rate
                                event_receiver_tx.send(InnerMessage::Poll(None)).ok();
                                if let Ok(InnerMessage::Poll(Some(poll))) = repl_rx.recv() {
                                    println!("Event polling rate is {} milliseconds", poll.as_millis());
                                } else {
                                    eprintln!("Event receiver thread failed to respond!");
                                }
                            } else {
                                // Send new polling rate
                                if let Ok(poll) = args[1].parse() {
                                    event_receiver_tx.send(InnerMessage::Poll(Some(Duration::from_millis(poll)))).ok();
                                    if repl_rx.recv().unwrap() == InnerMessage::Confirm {
                                        println!("Event polling rate set to {} milliseconds", poll);
                                    } else {
                                        eprintln!("Event receiver thread failed to confirm!");
                                    }
                                } else {
                                    println!("Failed to parse '{}'", args[1]);
                                }
                            }
                        },
                        "tell" => {

                        },
                        "tellall" => {

                        },
                        _ => {
                            println!("Unknown command. Type 'help' for usage")
                        },
                    }
                    rl.add_history_entry(line.as_str());
                }
            }
        }
        if rl.save_history("server.txt").is_err() {
            println!("Error, could not save command history");
        }
        event_receiver_tx.send(InnerMessage::Shutdown).ok();
    }
}

const REPL_HELP: &str = "\
COMMANDS
    quit : Shutdown server
    help : Print this message
    poll : ";
