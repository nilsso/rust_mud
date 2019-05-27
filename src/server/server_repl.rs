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
        let args = line.replace(|c| c == '\n' || c == '\r', "");
        let args: Vec<&str> = args.split_whitespace().collect();

        // Parse commands
        if !args.is_empty() {
            match args[0] {
                "quit" => break,
                "help" => {
                    println!("{}", HELP);
                    continue;
                }
                _ => {}
            }
        }

        // Parse scripts
        match args.as_slice() {
            _ => {
                println!("{}", HELP);
            }
        }
        rl.add_history_entry(line.as_str());
    }
    rl.save_history("history.txt").unwrap();
    Ok(())
}
