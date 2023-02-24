use std::{
    io,
    process::{Command, Output},
};

pub fn run_command(command: &str) -> Result<Output, io::Error> {
    Command::new("sh").arg("-c").arg(command).output()
}

pub fn search(query: &str, flag: &str) -> Vec<String> {
    let command = format!("xdotool search --onlyvisible {} {}", flag, query);
    let output = run_command(&command);

    match output {
        Ok(o) => {
            if o.status.success() {
                println!("Success message: {:#?}", o.stdout);
                String::from_utf8_lossy(&o.stdout)
                    .lines()
                    .map(|s| s.to_owned())
                    .collect()
            } else {
                eprintln!("Error message: {:#?}", String::from_utf8_lossy(&o.stderr));
                Vec::new()
            }
        }
        Err(err) => {
            eprintln!("Error message: {:#?}", err);
            Vec::new()
        }
    }
}

pub fn center_window(window_id: &str) {
    let command = format!("xdotool windowmove {} 780 400", window_id);
    let output = run_command(&command);

    match output {
        Ok(o) => println!("Command executed successfully. Output: {:#?}", o),
        Err(err) => eprintln!("Command execution failed with error: {:#?}", err),
    }
}

pub fn get_window_name(window_id: &str) -> String {
    let command = format!("xdotool getwindowname {}", window_id);
    let output = run_command(&command);

    match output {
        Ok(o) => String::from_utf8_lossy(&o.stdout).trim().to_string(),
        Err(err) => {
            eprintln!("Error message: {:#?}", err.to_string());
            String::new()
        }
    }
}

pub fn activate_window(window_id: &str) {
    let command = format!("xdotool windowactivate {}", window_id);
    let output = run_command(&command);

    match output {
        Ok(o) => println!("Command executed successfully. Output: {:#?}", o),
        Err(err) => eprintln!("Command execution failed with error: {:#?}", err),
    }
}
