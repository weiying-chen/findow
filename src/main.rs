use glib::clone;
use gtk::gdk::Display;
use gtk::glib;
use gtk::prelude::*;

use gtk::{
    Application, ApplicationWindow, Box as Box_, CssProvider, Entry, Label, ListBox, Orientation,
    StyleContext, STYLE_PROVIDER_PRIORITY_APPLICATION,
};

use std::cell::RefCell;
use std::process::{Command, Output};
use std::rc::Rc;

fn main() {
    let app = Application::new(Some("com.weiyingchen.ui-demo"), Default::default());

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

fn run_command(command: &str) -> Output {
    Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .unwrap_or_else(|_| panic!("failed to execute {}'", command))
}

fn populate_list_box(window_ids: &str, list_box: &ListBox) {
    for window_id in window_ids.split("\n").filter(|s| !s.is_empty()) {
        let command = format!("xdotool getwindowname {}", window_id);
        let window_name = run_command(&command);

        let window_name = String::from_utf8_lossy(&window_name.stdout)
            .trim()
            .to_string();

        if !window_name.is_empty() {
            let label = Label::new(Some(&window_name));
            list_box.append(&label);
        }
    }
}

fn get_window_ids(text: &str) -> String {
    let command = format!("xdotool search --onlyvisible --name {}", text);
    let output = run_command(&command);
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

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
    let window_ids = get_window_ids(&text);

    populate_list_box(&window_ids, &list_box);

    let window = ApplicationWindow::new(app);

    window.set_title(Some("CSS"));
    window.set_child(Some(&vbox));

    window.connect_show(clone!(@weak window => move |_| {
        // This is necessary for the command to run.
        glib::idle_add(|| {
            let command = "wmctrl -r \"CSS\" -e 1,640,100,680,768";

            run_command(command);
            glib::Continue(false)
        });
    }));

    window.show();

    let window_ids_rc = Rc::new(RefCell::new(String::new()));
    let window_ids_clone = Rc::clone(&window_ids_rc);

    entry.connect_changed(move |entry| {
        clear_list_box(&list_box);

        let text = entry.text();
        let mut window_ids = window_ids_clone.borrow_mut();

        *window_ids = get_window_ids(&text);

        if window_ids.is_empty() {
            let text = "\"\"";

            *window_ids = get_window_ids(&text)
        }

        populate_list_box(&window_ids, &list_box);
    });

    let window_ids_clone = Rc::clone(&window_ids_rc);

    entry.connect_activate(clone!(@weak window => move |_| {
        let window_ids = window_ids_clone.borrow();
        let command = format!("xdotool windowactivate {}", window_ids);

        run_command(&command);

        window.hide();
        window.close();
    }));
}
