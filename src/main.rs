use glib_macros::clone;
use gtk::glib;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Orientation};
use std::process::Command;

fn main() {
    let app = Application::builder()
        .application_id("com.jwestall.ui-demo")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn run_command(command: &str) -> String {
    // let cmd = format!("{} {}", command, input_text);
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Failed to execute proccess.");

    if output.status.success() {
        String::from_utf8_lossy(&output.stdout).trim().to_owned()
    } else {
        String::from_utf8_lossy(&output.stderr).trim().to_owned()
    }
}

fn build_ui(app: &Application) {
    let input = gtk::Entry::builder()
        .placeholder_text("input")
        .margin_top(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let button = Button::builder()
        .label("Submit")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("gtk-app")
        .child(&gtk_box)
        .decorated(false)
        .build();

    gtk_box.append(&input);
    gtk_box.append(&button);

    input.connect_activate(clone!(@weak window => move |entry| {
        // TODO: This could be a function

        // "xdotool search --onlyvisible --name {}",
        // "xdotool getwindowname {}",
        // "xprop -id {} | grep WM_CLASS",

        let input_text = entry.text();
        let command = format!("xdotool search --onlyvisible --name {}", input_text);
        let window_id_output = run_command(&command);

        println!("window_id: {}", window_id_output);

        let command = format!("xdotool getwindowname {}", window_id_output);
        let window_name_output = run_command(&command);

        println!("window_name: {}", window_name_output);

        // let command = "xprop -id {} | grep WM_CLASS";
        // let window_class_output = run_command(&command, &window_id_output);

        // println!("window_class: {}", window_class_output);
        window.close();
    }));

    window.present();
}
