#![allow(unused_imports)]
use std::io::{self, Write, BufRead, BufReader, Read};
use std::process::exit;
use std::env;
use std::path::{Path, PathBuf};
use std::fs::{self, OpenOptions};
use std::env::args;

mod commands;

fn main() {
    let stdin = io::stdin();

    loop {
        // Print prompt
        print!("$ ");
        io::stdout().flush().unwrap();

        // Read input
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input = input.trim();


        // Tokenize
        let words: Vec<&str> = input.split_whitespace().collect();

        match words.as_slice() {
            [""] => continue,
            ["echo", args @ ..] => commands::echo::cmd_echo(args),
            ["type", args @ ..] => commands::cmd_type::cmd_type(args),
            ["history"] => cmd_history(),
            ["exit", "session"] => {
                println!("Terminating the current session...");
                exit(0);
            }
            _ => {
                let word_strings: Vec<String> = words.iter().map(|s| s.to_string()).collect();
                if let Err(e) = commands::path_cmd::run_path_cmd_wrapper(&word_strings) {
                    eprintln!("Error: {}", e);
                }
            }
        }
    }
}
//fn create_file() -> std::io::Result<()> {
//    let mut file = File::create("foo.txt")?;
//    file.write_all(b"Hello there")?;
//    Ok(())
//}

fn cmd_history() {
    
}
