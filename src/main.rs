use glib_macros::clone;
use gtk::glib;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Orientation};
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

    input.connect_activate(clone!(@weak window => move |entry| {
        let input_text = entry.text();

        let output = Command::new("wmctrl")
            .arg("-a")
            .arg(input_text)
            .output()
            .expect("Failed to execute wmctrl command");

        println!("output.stdout: {}", String::from_utf8_lossy(&output.stdout));

        let output = Command::new("wmctrl")
            .arg("-l")
            .output()
            .expect("Failed to execute wmctrl command");

        println!("output.stdout: {}", String::from_utf8_lossy(&output.stdout));

        // `window.close()` sometimes won't bring the matched window to the front.
        window.hide();
    }));

    window.show();
}
