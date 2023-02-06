use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
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

    let window = ApplicationWindow::builder()
        .application(app)
        .title("gtk-app")
        .child(&input)
        .build();

    window.show_all();

    input.connect_activate(move |entry| {
        let input_text = entry.text();
        println!("ls {}", input_text);

        match Command::new("ls").arg(input_text).output() {
            Ok(output) => {
                if output.status.success() {
                    print!("output: {}", String::from_utf8_lossy(&output.stdout));
                    window.hide();
                    window.close();
                } else {
                    println!("output error: {}", output.status);
                }
            }
            Err(e) => {
                println!("Err: {}", e);
            }
        }
    });
}
