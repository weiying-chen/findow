use std::process::{Command, Output};

pub fn run_command(command: &str) -> Output {
    Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .unwrap_or_else(|_| panic!("Failed to execute `{}`.", command))
}

pub fn search(query: &str, flag: &str) -> Vec<String> {
    let command = format!("xdotool search --onlyvisible {} {}", flag, query);
    let output = run_command(&command);

    if output.status.success() {
        String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|s| s.to_owned())
            .collect()
    } else {
        eprintln!(
            "Error message: {:?}",
            String::from_utf8_lossy(&output.stderr).trim().to_owned()
        );
        Vec::new()
    }
}

pub fn get_window_name(window_id: &str) -> String {
    let command = format!("xdotool getwindowname {}", window_id);
    let output = run_command(&command);

    String::from_utf8_lossy(&output.stdout).trim().to_string()
}
