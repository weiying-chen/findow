use std::{
    fmt, io,
    process::{Command, Output},
};

#[derive(Debug)]
enum CommandError {
    CouldNotExecute {
        command: String,
        source: std::io::Error,
    },

    NonZeroExit {
        command: String,
        status: std::process::ExitStatus,
        stderr: Vec<u8>,
        stdout: Vec<u8>,
    },
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::CouldNotExecute { source, command } => {
                write!(f, "Could not execute command '{}': {}", command, source)
            }
            CommandError::NonZeroExit {
                status,
                command,
                stderr,
                stdout,
            } => {
                let stderr_str = String::from_utf8_lossy(stderr).trim().to_owned();
                let stdout_str = String::from_utf8_lossy(stdout).trim().to_owned();
                writeln!(f, "Command `{}` failed with status {}", command, status)?;
                writeln!(f, "stderr: {}", stderr_str)?;
                writeln!(f, "stdout: {}", stdout_str)?;
                Ok(())
            }
        }
    }
}

pub fn run_command(command: &str) -> Result<Output, io::Error> {
    Command::new("sh").arg("-c").arg(command).output()

    // Command::new("wrongcommand").arg("non-existent-file").output()
}

pub fn search(query: &str, flag: &str) -> Vec<String> {
    let command = format!("xdotool search --onlyvisible {} {}", flag, query);

    run_command(&command)
        .map_err(|err| CommandError::CouldNotExecute {
            source: err,
            command: command.to_owned(),
        })
        .and_then(|output| {
            if output.status.success() {
                Ok(output)
            } else {
                Err(CommandError::NonZeroExit {
                    status: output.status,
                    command: command.to_owned(),
                    stderr: output.stderr,
                    stdout: output.stdout,
                })
            }
        })
        .map_or_else(
            |err| {
                eprintln!("Error: {}", err);
                Vec::new()
            },
            |output| {
                let stdout = String::from_utf8_lossy(&output.stdout);

                stdout.lines().map(|s| s.to_owned()).collect()
            },
        )
}

pub fn center_window(window_id: &str) {
    let command = format!("xdotool windowmove {} 780 400", window_id);
    let output = run_command(&command);

    match output {
        // TODO: Change the name of o
        Ok(o) => println!(
            "{} executed successfully. Output: {}",
            command,
            String::from_utf8_lossy(&o.stdout)
        ),
        Err(err) => eprintln!("Command execution failed with error: {}", err),
    }
}

pub fn get_window_name(window_id: &str) -> String {
    let command = format!("xdotool getwindowname {}", window_id);
    let output = run_command(&command);

    match output {
        // TODO: Change the name of o
        Ok(o) => String::from_utf8_lossy(&o.stdout).trim().to_string(),
        Err(err) => {
            eprintln!("Error message: {}", err.to_string());
            String::new()
        }
    }
}

pub fn activate_window(window_id: &str) {
    let command = format!("xdotool windowactivate {}", window_id);
    let output = run_command(&command);

    match output {
        Ok(o) => println!(
            "Command executed successfully. Output: {}",
            String::from_utf8_lossy(&o.stdout)
        ),
        Err(err) => eprintln!("Command execution failed with error: {}", err),
    }
}
