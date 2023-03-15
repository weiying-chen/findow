use std::{
    fmt, io,
    process::{Command, Output},
};

use crate::constants::WINDOW_WIDTH;

#[derive(Debug)]
enum CommandError {
    ExecutionError {
        source: std::io::Error,
    },

    NonZeroExit {
        status: std::process::ExitStatus,
        stderr: Vec<u8>,
        stdout: Vec<u8>,
    },
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::ExecutionError { source } => {
                write!(f, "Source: {}", source)
            }
            CommandError::NonZeroExit {
                status,
                stderr,
                stdout,
            } => {
                let stderr_str = String::from_utf8_lossy(stderr).trim().to_owned();
                let stdout_str = String::from_utf8_lossy(stdout).trim().to_owned();
                writeln!(f, "status: {}", status)?;
                writeln!(f, "stderr: {}", stderr_str)?;
                writeln!(f, "stdout: {}", stdout_str)?;
                Ok(())
            }
        }
    }
}

fn create_execution_error(err: io::Error) -> CommandError {
    CommandError::ExecutionError { source: err }
}

fn handle_output(output: Output) -> Result<Output, CommandError> {
    if output.status.success() {
        Ok(output)
    } else {
        Err(CommandError::NonZeroExit {
            status: output.status,
            stderr: output.stderr,
            stdout: output.stdout,
        })
    }
}

fn run_command(command: &str) -> Result<Output, CommandError> {
    Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .map_err(|err| create_execution_error(err))
        .and_then(|output| handle_output(output))
}

pub fn search_windows(options: &str, pattern: &str) -> Vec<String> {
    let command = format!("xdotool search --onlyvisible {} {}", options, pattern);

    run_command(&command).map_or_else(
        |err| {
            eprintln!("Command: {}", command);
            eprintln!("Error: \n{}", err);
            Vec::new()
        },
        |output| {
            let stdout = String::from_utf8_lossy(&output.stdout);

            stdout.lines().map(|s| s.to_owned()).collect()
        },
    )
}

pub fn center_window(window_id: &str) {
    let command = format!("xdpyinfo | grep dimensions | awk '{{print $2}}'");

    let output = run_command(&command).map_or_else(
        |err| {
            eprintln!("Command: {}", command);
            eprintln!("Error: \n{}", err);
            String::new()
        },
        |output| String::from_utf8_lossy(&output.stdout).trim().to_owned(),
    );

    let dimensions = output
        .trim()
        .split("x")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let screen_width = dimensions[0];

    let window_width = WINDOW_WIDTH;
    let x = (screen_width - window_width) / 2;
    let y = 100;
    let command = format!("xdotool windowmove {} {} {}", window_id, x, y);

    if let Err(err) = run_command(&command) {
        eprintln!("Command: {}", command);
        eprintln!("Error: \n{}", err);
    }
}

pub fn get_window_name(window_id: &str) -> String {
    let command = format!("xdotool getwindowname {}", window_id);

    run_command(&command).map_or_else(
        |err| {
            eprintln!("Command: {}", command);
            eprintln!("Error: \n{}", err);
            String::new()
        },
        |output| String::from_utf8_lossy(&output.stdout).trim().to_owned(),
    )
}

pub fn activate_window(window_id: &str) {
    let command = format!("xdotool windowactivate {}", window_id);

    if let Err(err) = run_command(&command) {
        eprintln!("Command: {}", command);
        eprintln!("Error: \n{}", err);
    }
}
