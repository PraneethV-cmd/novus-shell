#![allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;
use std::env;
use std::path::{Path, PathBuf};

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

        // Exit command
        if input == "exit 0" {
            break;
        }

        // Tokenize
        let words: Vec<&str> = input.split_whitespace().collect();

        match words.as_slice() {
            [""] => continue,
            ["echo", args @ ..] => cmd_echo(args),
            ["type", args @ ..] => cmd_type(args),
            ["exit", "session"] => {
                println!("Terminating the current session...");
                exit(0);
            }
            _ => {
                if let Some(path) = explore_path(words[0]) {
                    println!("{} is {}", words[0], path.display());
                } else {
                    println!("{}: command not found", words[0]);
                }
            }
        }
    }
}

fn cmd_echo(args: &[&str]) {
    println!("{}", args.join(" "));
}

fn cmd_type(args: &[&str]) {
    if args.is_empty() {
        return;
    }

    if args.len() > 1 {
        println!("Too many arguments...");
        return;
    }

    let cmd = args[0];

    match cmd {
        "type" | "echo" | "exit" => {
            println!("{} is a shell builtin", cmd);
        }
        _ => {
            if let Some(path) = explore_path(cmd) {
                println!("{} is {}", cmd, path.display());
            } else {
                println!("{}: Command not found...", cmd);
            }
        }
    }
}

fn explore_path<P>(exe_name: P) -> Option<PathBuf>
where
    P: AsRef<Path>,
{
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths)
            .filter_map(|dir| {
                let full_path = dir.join(&exe_name);
                if full_path.is_file() {
                    Some(full_path)
                } else {
                    None
                }
            }).next()
    })
}

