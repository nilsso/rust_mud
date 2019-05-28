//! Server module REPL (Read-eval-print loop)

// External code
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::io::{stdout, Write};
use std::str::FromStr;
use log::LevelFilter;

// Internal code
//use super::SERVER_LOGGER;

const HELP: &str = "\
COMMANDS:
    help      : Print this message
    quit      : Shutdown server
    log_level : Set server loggin level";
const LOG_LEVEL_VALID: &str = "Valid arguments: Off, Error, Warn, Info, Debug, Trace";

pub fn server_repl() -> Result<bool, ReadlineError> {
    let mut stdout = stdout();
    let mut rl = Editor::<()>::new();
    rl.load_history("history.txt").ok();
    let mut running = true;
    while running {
        let line = rl.readline("> ")?;
        let args = line.replace(|c| c == '\n' || c == '\r', "");
        let args: Vec<&str> = args.split_whitespace().collect();
        if !args.is_empty() {
            match args[0] {
                "quit" => running = false,
                "help" => println!("{}", HELP),
                "log_level" => {
                    if args.len() < 2 {
                        println!("Log level is {}", log::max_level());
                    } else {
                        match LevelFilter::from_str(args[1]) {
                            Ok(level_filter) => {
                                println!("Log filter now {}", level_filter);
                                log::set_max_level(level_filter);
                            }
                            Err(_) => println!(
                                "Failed to parse '{}' as level filter\n{}",
                                args[1], LOG_LEVEL_VALID
                            ),
                        };
                    }
                },
                "error" => error!("test!"),
                "warn" => warn!("test!"),
                "info" => info!("test!"),
                "debug" => debug!("test!"),
                "trace" => trace!("test!"),
                _ => println!("Unknown command. Type 'help' for usage"),
            }
        }
        rl.add_history_entry(line.as_str());
    }
    if rl.save_history("history.txt").is_err() {
        println!("Error, could not save command history");
    }
    Ok(true)
}
