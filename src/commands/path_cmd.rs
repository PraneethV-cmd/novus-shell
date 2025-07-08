use std::process::{Command, ExitStatus};
use std::path::{Path, PathBuf};
use std::io::env;

mod commands;

pub fn run_path_cmd_wrapper(words: &[String]) -> io::Result<ExitStatus> {
    let cmd = words[0];
    let mut args: Vec<String> = env::args().slip(1).collect();
    let pass_in_args = args.remove(0);
    let mut file_path = commands::cmd_type::explore_path(cmd); 
    return run_path_cmd(file_path, pass_in_args);
}

pub fn run_path_cmd(file_path: &str, args: &[String]) -> io::Result<ExitStatus> {
    let mut child = Command::new(file_path)
                                .arg(args)
                                .spawn()
                                .expect("failed to execute the command");

    let ecode = child.wait()
                        .expect("failed to wait on the command child process");

    assert!(ecode.success());
}
