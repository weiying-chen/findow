use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Label, ListBox, ListBoxRow};
use std::cell::RefCell;
use std::process::{Command, Output};
use std::rc::Rc;

fn main() {
    let app = Application::builder()
        .application_id("com.jwestall.ui-demo")
        .build();

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

fn print_output(name: &str, output: &Output) {
    if output.status.success() {
        println!("{} (success): {:?}", name, output);
    } else {
        println!("{} (failure): {:?}", name, output);
    }
}

fn build_ui(app: &Application) {
    let label = Label::new(None);
    let list_box = ListBox::new();
    let list_box_row = ListBoxRow::new();
    let vbox = Box::new(gtk::Orientation::Vertical, 10);

    let input = gtk::Entry::builder()
        .placeholder_text("input")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    list_box_row.add(&label);
    list_box.add(&list_box_row);

    vbox.pack_start(&input, false, false, 0);
    vbox.pack_start(&list_box, true, true, 0);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("gtk-app")
        .child(&vbox)
        .build();

    let window_id_output_rc = Rc::new(RefCell::new(String::new()));
    let window_id_output_clone = Rc::clone(&window_id_output_rc);

    window.show_all();

    input.connect_changed(move |entry| {
        let mut window_id_output_string = window_id_output_clone.borrow_mut();
        let input_text = entry.text();
        let command = format!("xdotool search --onlyvisible --name {}", input_text);
        let window_id_output = run_command(&command);

        print_output("window_id_output", &window_id_output);
        *window_id_output_string = String::from_utf8_lossy(&window_id_output.stdout).to_string();

        label.set_text("Test text");

        // TODO: Check the result of the split
        // See if can use filter like suggested in Reddit

        // for window_id in window_id_output_string.split("\n") {
        //     if !window_id.is_empty() {
        //         // TODO: Should fix this because because it fails when `window_id_output` has two ids.
        //         let command = format!("xdotool getwindowname {}", window_id);
        //         let window_name_output = run_command(&command);
        //         let window_name = String::from_utf8_lossy(&window_name_output.stdout).to_string();
        //         let label = Label::new(None);

        //         label.set_text(&window_name);

        //         let list_box_row = ListBoxRow::new();

        //         list_box_row.add(&label);
        //         list_box.add(&list_box_row);
        //     }
        // }
    });

    let window_id_output_clone = Rc::clone(&window_id_output_rc);

    input.connect_activate(move |_| {
        let window_id_output_string = window_id_output_clone.borrow();
        let command = format!("xdotool windowactivate {}", window_id_output_string);
        let window_activate_output = run_command(&command);

        print_output("window_activate_output", &window_activate_output);
        window.hide();
        window.close();
    });
}
