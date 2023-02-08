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

fn print_output(name: &str, output: &Output) {
    if output.status.success() {
        println!("{} (success): {:?}", name, output);
    } else {
        println!("{} (failure): {:?}", name, output);
    }
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
        let mut window_id_output_string = window_id_output_clone.borrow_mut();
        let input_text = entry.text();
        let command = format!("xdotool search --onlyvisible --name {}", input_text);
        let window_id_output = run_command(&command);

        print_output("window_id_output", &window_id_output);
        *window_id_output_string = String::from_utf8_lossy(&window_id_output.stdout).to_string();

        // TODO: This fails when window_id_output has two ids
        let command = format!("xdotool getwindowname {}", window_id_output_string);
        let window_name_output = run_command(&command);

        print_output("window_name_output", &window_name_output);
    });

    let window_id_output_clone = Rc::clone(&window_id_output_rc);

    input.connect_activate(move |_| {
        let window_id_output_string = window_id_output_clone.borrow();
        let command = format!("xdotool windowactivate {}", window_id_output_string);
        let window_activate_output = run_command(&command);

        print_output("window_activate_output", &window_activate_output);
        window.hide();
        window.close();
    });
}
