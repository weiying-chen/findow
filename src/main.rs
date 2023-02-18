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

    app.connect_startup(|app| {
        let provider = CssProvider::new();

        provider.load_from_data(include_str!("style.css"));

        StyleContext::add_provider_for_display(
            &Display::default().expect("Could not connect to a display."),
            &provider,
            STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        build_ui(app);
    });

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
        println!("{} (success): {:#?}", name, output);
    } else {
        println!("{} (failure): {:#?}", name, output);
    }
}

fn populate_list_box(window_id_output_string: &str, list_box: &ListBox) {
    for window_id in window_id_output_string
        .split("\n")
        .filter(|s| !s.is_empty())
    {
        let command = format!("xdotool getwindowname {}", window_id);
        let window_name_output = run_command(&command);

        let window_name = String::from_utf8_lossy(&window_name_output.stdout)
            .trim()
            .to_string();

        if !window_name.is_empty() {
            let label = Label::new(Some(&window_name));
            list_box.append(&label);
        }
    }
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

    let input_text = "\"\"";
    let command = format!("xdotool search --onlyvisible --name {}", input_text);
    let window_id_output = run_command(&command);
    let window_id_output_string = String::from_utf8_lossy(&window_id_output.stdout).to_string();

    print_output("window_id_output", &window_id_output);
    populate_list_box(&window_id_output_string, &list_box);

    let window = ApplicationWindow::new(app);

    window.set_title(Some("CSS"));
    window.set_child(Some(&vbox));

    app.connect_activate(clone!(@weak window => move |_| {
        window.show();
    }));

    let window_id_output_rc = Rc::new(RefCell::new(String::new()));
    let window_id_output_clone = Rc::clone(&window_id_output_rc);

    entry.connect_changed(move |entry| {
        let input_text = entry.text();
        let command = format!("xdotool search --onlyvisible --name {}", input_text);
        let window_id_output = run_command(&command);

        print_output("window_id_output", &window_id_output);
        clear_list_box(&list_box);

        let mut window_id_output_string = window_id_output_clone.borrow_mut();

        *window_id_output_string = String::from_utf8_lossy(&window_id_output.stdout).to_string();

        println!("window_id_output_string: {}", window_id_output_string);

        if window_id_output_string.is_empty() {
            let input_text = "\"\"";
            let command = format!("xdotool search --onlyvisible --name {}", input_text);
            let window_id_output = run_command(&command);

            *window_id_output_string =
                String::from_utf8_lossy(&window_id_output.stdout).to_string();
        }

        populate_list_box(&window_id_output_string, &list_box);
    });

    let window_id_output_clone = Rc::clone(&window_id_output_rc);

    entry.connect_activate(clone!(@weak window => move |_| {
        let window_id_output_string = window_id_output_clone.borrow();
        let command = format!("xdotool windowactivate {}", window_id_output_string);
        let window_activate_output = run_command(&command);

        print_output("window_activate_output", &window_activate_output);
        window.hide();
        window.close();
    }));
}
