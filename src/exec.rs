use std::ffi::OsStr;
use std::io::{BufRead, BufReader};
use std::process::{Command, ExitStatus, Stdio};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExecError {
    #[error("spawn failure: {0}")]
    SpawnFailure(String),
    #[error("exec failed: {command} {status}")]
    Failed { command: String, status: ExitStatus },
}

pub fn exec<S: AsRef<OsStr>>(cmd: S, args: Vec<S>) -> Result<(), ExecError> {
    let mut command_list: Vec<String> = vec![cmd.as_ref().to_string_lossy().into_owned()];
    for arg in args.iter() {
        let a = arg.as_ref().to_string_lossy().into_owned();
        command_list.push(a);
    }
    let command = command_list.join(" ");

    println!("> {}", command);

    let mut child = Command::new(cmd)
        .args(args)
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|_| ExecError::SpawnFailure(command.clone()))?;

    let stderr = child.stderr.take().unwrap();
    let err_reader = BufReader::new(stderr);
    for line in err_reader.lines() {
        println!("{}", line.unwrap());
    }

    let status = child
        .wait()
        .map_err(|_| ExecError::SpawnFailure(command.clone()))?;

    if !status.success() {
        Err(ExecError::Failed { command, status })
    } else {
        Ok(())
    }
}
