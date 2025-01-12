use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};
use std::cell::RefCell;
use std::rc::Rc;

const APP_ID: &str = "com.example.gui";

fn main() {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(application: &Application) {
    // Create a new window
    let window = ApplicationWindow::builder()
        .application(application)
        .title("Counter")
        .default_width(300)
        .default_height(100)
        .build();

    // Create a new button
    let button = Button::builder()
        .label("Increase")
        .margin_top(20)
        .margin_bottom(20)
        .margin_start(20)
        .margin_end(20)
        .build();

    // Create a counter
    let counter = Rc::new(RefCell::new(0));

    // Connect to "clicked" signal of `button`
    let window_clone = window.clone();
    let counter_clone = Rc::clone(&counter);
    button.connect_clicked(move |_| {
        *counter_clone.borrow_mut() += 1;
        println!("Counter: {}", *counter_clone.borrow());
        window_clone.set_title(Some("You clicked"));
    });

    // Add the button to the window
    window.set_child(Some(&button));

    // Present the window
    window.present();
}
