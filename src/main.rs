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

    let window_id_output_rc = Rc::new(RefCell::new(String::new()));
    let window_id_output_clone = Rc::clone(&window_id_output_rc);

    window.show_all();

    input.connect_changed(move |entry| {
        let input_text = entry.text();
        let command = format!("xdotool search --onlyvisible --name {}", input_text);
        let window_id_output = run_command(&command);
        let mut window_id_output_string = window_id_output_clone.borrow_mut();

        if window_id_output.status.success() {
            *window_id_output_string =
                String::from_utf8_lossy(&window_id_output.stdout).to_string();
            println!("window_id_output (success): {:?}", window_id_output);
        } else {
            println!("window_id-output (failure): {:?}", window_id_output);
        }
    });

    let window_id_output_clone = Rc::clone(&window_id_output_rc);

    input.connect_activate(move |_| {
        let window_id_output_string = window_id_output_clone.borrow();
        let command = format!("xdotool windowactivate {}", window_id_output_string);
        let window_activate_output = run_command(&command);

        println!("window_activate: {:?}", window_activate_output);
        window.hide();
        window.close();
    });
}
