use std::process::{Command, ExitStatus};
use std::path::{Path, PathBuf};
use std::env;
use std::io;

use crate::commands::cmd_type;

pub fn run_path_cmd_wrapper(words: &[String]) -> io::Result<ExitStatus> {
    let cmd = &words[0];
    let args = &words[1..];

    let file_path = match cmd_type::explore_path(cmd) {
        Some(path) => path,
        None => return Err(io::Error::new(io::ErrorKind::NotFound, "command not found")),
    };

    let path_str = file_path.to_str().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidData, "Path is not valid UTF-8")
    })?;

    run_path_cmd(path_str, args)
}

pub fn run_path_cmd(file_path: &str, args: &[String]) -> io::Result<ExitStatus> {
    let mut child = Command::new(file_path)
                                .args(args)
                                .spawn()
                                .expect("failed to execute the command");

    let ecode = child.wait()
                        .expect("failed to wait on the command child process");

    assert!(ecode.success());
    Ok(ecode)
}
