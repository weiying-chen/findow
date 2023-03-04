use glib::clone;
use gtk::gdk::Display;
use gtk::glib;
use gtk::prelude::*;
use ui_demo::xdotool;

use gtk::{
    Application, ApplicationWindow, Box as Box_, CssProvider, Entry, Label, ListBox, Orientation,
    StyleContext, STYLE_PROVIDER_PRIORITY_APPLICATION,
};

use std::cell::RefCell;
// use std::process::Command;
// use std::process::{Command, Output};
use std::rc::Rc;

const APP_ID: &str = "com.weiyingchen.ui-demo";

fn main() -> glib::ExitCode {
    let app = Application::new(Some(APP_ID), Default::default());

    app.connect_startup(|_| {
        let provider = CssProvider::new();

        provider.load_from_data(include_str!("style.css"));

        StyleContext::add_provider_for_display(
            &Display::default().expect("Could not connect to a display."),
            &provider,
            STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    });

    app.connect_activate(build_ui);
    app.run()
}

// fn run_command(command: &str) -> Output {
//     Command::new("sh")
//         .arg("-c")
//         .arg(command)
//         .output()
//         .unwrap_or_else(|_| panic!("failed to execute {}'", command))
// }

fn populate_list_box(window_ids: &Vec<String>, list_box: &ListBox) {
    for window_id in window_ids.iter().filter(|s| !s.is_empty()) {
        let window_name = xdotool::get_window_name(window_id);

        if !window_name.is_empty() {
            let label = Label::new(Some(&window_name));
            list_box.append(&label);
        }
    }
}

// fn search_windows(text: &str) -> String {
//     let command = format!("xdotool search --onlyvisible --name {}", text);
//     let output = run_command(&command);
//     String::from_utf8_lossy(&output.stdout).trim().to_string()
// }

fn clear_list_box(list_box: &ListBox) {
    while let Some(row) = list_box.last_child() {
        list_box.remove(&row);
    }
}

fn build_ui(app: &Application) {
    let vbox = Box_::new(Orientation::Vertical, 0);
    let entry = Entry::new();

    vbox.append(&entry);

    let list_box = ListBox::new();

    vbox.append(&list_box);

    let pattern = "\"\"";
    let window_ids = xdotool::search_windows("--name", &pattern);

    populate_list_box(&window_ids, &list_box);

    let window = ApplicationWindow::new(app);

    const WINDOW_NAME: &str = "CSS";

    window.set_title(Some(WINDOW_NAME));
    window.set_child(Some(&vbox));

    window.connect_realize(clone!(@weak window => move |_| {
        // TODO: Fix this ussing xb11rb
        // std::thread::sleep(std::time::Duration::from_secs(1)); // add a delay of 1 second
        // let window_id = xdotool::search(WINDOW_NAME, "--name");

        // let command = format!("xdotool search --onlyvisible --name {}", WINDOW_NAME);

        // let window_id = Command::new("sh").arg("-c").arg(command).output();


        // let window_id = xdotool::search(WINDOW_NAME, "--name");

        // println!("window_id: {:?}", window_id);
        // xdotool::center_window(&window_id.join(", "));
    }));

    window.show();

    let window_ids_rc = Rc::new(RefCell::new(Vec::new()));
    let window_ids_clone = Rc::clone(&window_ids_rc);

    entry.connect_changed(move |entry| {
        clear_list_box(&list_box);

        let pattern = entry.text();
        let mut window_ids = window_ids_clone.borrow_mut();

        *window_ids = xdotool::search_windows("--name", &pattern);

        if window_ids.is_empty() {
            let pattern = "\"\"";

            *window_ids = xdotool::search_windows("--name", &pattern);
        }

        populate_list_box(&window_ids, &list_box);
    });

    let window_ids_clone = Rc::clone(&window_ids_rc);

    entry.connect_activate(clone!(@weak window => move |_| {
        let window_ids = window_ids_clone.borrow();

        // If there are more than one window, the first one that matches will be activated.
        xdotool::activate_window(&window_ids.join(", "));
        // window.hide();
        // window.close();

        window.minimize();
    }));
}
