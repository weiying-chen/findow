use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use std::process::{Command, Output};

fn main() {
    let app = Application::builder()
        .application_id("com.jwestall.ui-demo")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn wmctrl(args: &str) -> Output {
    Command::new("sh")
        .arg("-c")
        .arg(format!("wmctrl {}", args))
        .output()
        .unwrap_or_else(|_| panic!("Failed to execute `wmctrl {}`", args))
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

    input.connect_activate(move |entry| {
        let input_text = entry.text();

        println!("input_text: {}", input_text);

        let output = wmctrl(&format!("-a {}", input_text));

        if output.status.success() {
            println!("output: {}", String::from_utf8_lossy(&output.stdout));
            window.hide();
            window.close();
        } else {
            println!("output error: {}", output.status);
        }
    });
}
