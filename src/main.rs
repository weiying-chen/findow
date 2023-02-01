use glib_macros::clone;
use gtk::glib;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Orientation};

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
        let cmd = format!(
            "xdotool search --onlyvisible --name {} windowactivate",
            input_text
        );

        std::process::Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .spawn()
            .unwrap();

        window.close();
    }));
    window.present();
}
