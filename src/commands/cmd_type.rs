use std::io::{self, Write};
use std::env;
use std::path::{Path, PathBuf};

mod commands;

pub fn cmd_type(args: &[&str]) {
    if args.is_empty() {
        return;
    }
//
//    if args.len() > 1 {
//        println!("Too many arguments...");
//        return;
//    }
//
    let cmd = args[0];
    let mut args: Vec<String> = env::args().skip(1).collect();
    let pass_in_args = args.remove(0);

    match cmd {
        "type" | "echo" | "exit" => {
            println!("{} is a shell builtin...", cmd);
        }
        _ => {
            if let Some(path) = explore_path(cmd) {
                commads::path_cmd::run_path_cmd(path, pass_in_args);
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
