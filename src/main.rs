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
// use std::process::{Command, Output};
use std::rc::Rc;

const APP_ID: &str = "com.weiyingchen.ui-demo";

fn main() {
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
    app.run();
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

    let text = "\"\"";
    let window_ids = xdotool::search(&text, "--name");

    populate_list_box(&window_ids, &list_box);

    let window = ApplicationWindow::new(app);

    const WINDOW_NAME: &str = "CSS";

    window.set_title(Some(WINDOW_NAME));
    window.set_child(Some(&vbox));

    window.connect_show(clone!(@weak window => move |_| {
        // This is necessary for the command to run.
        glib::idle_add(|| {
            let window_id = xdotool::search(WINDOW_NAME, "--name");

            let command = format!("xdotool windowmove {} 780 400", window_id.join(", "));

            xdotool::run_command(&command);
            glib::Continue(false)
        });
    }));

    window.show();

    let window_ids_rc = Rc::new(RefCell::new(Vec::new()));
    let window_ids_clone = Rc::clone(&window_ids_rc);

    entry.connect_changed(move |entry| {
        clear_list_box(&list_box);

        let text = entry.text();
        let mut window_ids = window_ids_clone.borrow_mut();

        *window_ids = xdotool::search(&text, "--name");

        if window_ids.is_empty() {
            let text = "\"\"";

            *window_ids = xdotool::search(&text, "---name")
        }
        populate_list_box(&window_ids, &list_box);
    });

    let window_ids_clone = Rc::clone(&window_ids_rc);

    entry.connect_activate(clone!(@weak window => move |_| {
        let window_ids = window_ids_clone.borrow();
        let command = format!("xdotool windowactivate {}", window_ids.join(", "));

        xdotool::run_command(&command);

        window.hide();
        window.close();
    }));
}
