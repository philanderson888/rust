use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Label, Box};
use std::cell::RefCell;
use std::rc::Rc;
use gtk::{CssProvider,gdk::Display};

const APP_ID: &str = "com.example.gui";

fn main() {

    println!("==============================================================");
    println!("==============================================================");
    println!("====               Counter Sample Application              ====");
    println!("==============================================================");
    println!("==============================================================");

    println!("==============================================================");
    println!("====                   Introduction                       ====");
    println!("==============================================================");

    println!("Counter is a simple GUI application that increments a counter each time a button is clicked");   

    println!("\n... the idea will be to combine the learning to date to build a real-world application");

    println!("\ninputs: button click");
    println!("\noutputs: counter value");
    println!("\nconfiguration: environment variables, command-line arguments ... ");

    println!("\ndocumentation at https://www.gtk.org/docs/language-bindings/rust/");
    println!("\n... book at https://gtk-rs.org/gtk4-rs/stable/latest/book/widgets.html");

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("style.css"));

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}


fn build_ui(application: &Application) {
    let window = ApplicationWindow::builder()
        .application(application)
        .title("Counter")
        .default_width(500)
        .default_height(500)
        .build();
    window.set_widget_name("counter_window");

    let vbox = Box::new(gtk::Orientation::Vertical, 5);

    let button = Button::builder()
        .label("Increase")
        .margin_top(20)
        .margin_bottom(20)
        .margin_start(20)
        .margin_end(20)
        .build();
    button.set_widget_name("counter_button");
    button.add_css_class("text_button");

    let label = Label::new(Some("Counter: 0"));
    label.set_widget_name("counter_label");

    let counter = Rc::new(RefCell::new(0));

    let window_clone = window.clone();
    let counter_clone = Rc::clone(&counter);
    let label_clone = label.clone();
    let button_clone = button.clone();
    button.connect_clicked(move |_| {
        *counter_clone.borrow_mut() += 1;
        println!("Counter: {}", *counter_clone.borrow());
        window_clone.set_title(Some(&format!("Counter: {}", *counter_clone.borrow())));
        label_clone.set_text(&format!("Counter: {}", *counter_clone.borrow()));
        button_clone.set_label(&format!("Counter: {}", *counter_clone.borrow())); 
    });

    vbox.append(&button);
    vbox.append(&label);
    window.set_child(Some(&vbox));
    window.present();
}
