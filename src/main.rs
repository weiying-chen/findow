use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use std::cell::RefCell;
use std::process::{Command, Output};
use std::rc::Rc;

fn main() {
    let app = Application::builder()
        .application_id("com.jwestall.ui-demo")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn run_command(command: &str) -> Output {
    Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .unwrap_or_else(|_| panic!("failed to execute {}'", command))
}

fn build_ui(app: &Application) {
    let input = gtk::Entry::builder()
        .placeholder_text("input")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("gtk-app")
        .child(&input)
        .build();

    window.show_all();

    let shared_var = Rc::new(RefCell::new(String::new()));
    let shared_var_ref = Rc::clone(&shared_var);

    input.connect_changed(move |entry| {
        let input_text = entry.text();

        let command = format!("xdotool search --onlyvisible --name {}", input_text);
        let window_id_output = run_command(&command);

        if window_id_output.status.success() {
            println!(
                "stdout: {}",
                String::from_utf8_lossy(&window_id_output.stdout)
            );

            let mut shared = shared_var_ref.borrow_mut();

            *shared = String::from_utf8_lossy(&window_id_output.stdout).to_string()
        } else {
            println!(
                "sterr: {}",
                String::from_utf8_lossy(&window_id_output.stderr)
            );
        }
    });

    let shared_var_ref = Rc::clone(&shared_var);

    input.connect_activate(move |entry| {
        let input_text = entry.text();
        let shared = shared_var_ref.borrow();

        // // `xdotool windowactivate` doesn't produce any output
        let command = format!("xdotool windowactivate {:?}", shared);
        let window_activate_output = run_command(&command);

        println!("window_activate: {:?}", window_activate_output);

        window.hide();
        window.close();
    });
}
