use std::io::{self, Write};
use std::env;
use std::path::{Path, PathBuf};

pub fn cmd_type(args: &[&str]) {
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
            println!("{} is a shell builtin...", cmd);
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
where P: AsRef<Path>, {
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
