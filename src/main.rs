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
        let input_text = entry.text();

        // TODO: This could be a function

        let cmd = format!(
            "xdotool search --onlyvisible --name {}",
            input_text
        );

        let window_id_output = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .expect("Failed to execute proccess.");

        if window_id_output.status.success() {
            println!("window_id: {}", String::from_utf8_lossy(&window_id_output.stdout).trim());
        } else {
            println!("stderr: {}", String::from_utf8_lossy(&window_id_output.stderr).trim());
        }

        let cmd = format!(
            "xdotool getwindowname {}",
            String::from_utf8_lossy(&window_id_output.stdout).trim()
        );

        let window_name_output = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .expect("Failed to execute proccess.");

        if window_name_output.status.success() {
            println!("window_name: {}", String::from_utf8_lossy(&window_name_output.stdout).trim());
        } else {
            println!("stderr: {}", String::from_utf8_lossy(&window_name_output.stderr).trim());
        }

        let cmd = format!(
            "xprop -id {} | grep WM_CLASS",
            String::from_utf8_lossy(&window_id_output.stdout).trim()
        );

        let window_class_output = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .expect("Failed to execute proccess.");

        if window_class_output.status.success() {
            println!("window_class: {}", String::from_utf8_lossy(&window_class_output.stdout).trim());
        } else {
            println!("stderr: {}", String::from_utf8_lossy(&window_class_output.stderr).trim());
        }

        window.close();
    }));

    window.present();
}
