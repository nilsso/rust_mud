//! Server module REPL (Read-eval-print loop)

use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::io::{stdout, Write};

const HELP: &str = "\
COMMANDS:
    help  :  Print this message
    quit  :  Shutdown server";

pub fn server_repl() -> Result<(), ReadlineError> {
    let mut stdout = stdout();
    let mut rl = Editor::<()>::new();
    rl.load_history("history.txt").ok();
    loop {
        let line = rl.readline("> ")?;
        // TODO: Can I accomplish this in one line?
//        let args = line.replace(|c| c == '\n' || c == '\r', "");
//        let args: Vec<&str> = args.split_whitespace().collect();
        let args: Vec<String> = line.replace(|c| c == '\n' || c == '\r', "").split_whitespace().map(|s| s.to_string()).collect();
        if !args.is_empty() {
            match args[0].as_str() {
                "quit" => break,
                "help" => println!("{}", HELP),
                "log_max_level" => {
                    if args.len() > 1 {
                        match args[1].parse::<log::LevelFilter>() {
                            Ok(level_filter) => log::set_max_level(level_filter),
                            Err(e) => println!("Failed to parse {} as level filter: {:?}", args[1], e)
                        };
                    }
                },
                _ => {
                    println!("{}", HELP);
                }
            }
        }
        rl.add_history_entry(line.as_str());
    }
    rl.save_history("history.txt").unwrap();
    Ok(())
}
