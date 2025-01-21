use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Label};
use gtk::{CssProvider,gdk::Display};
use gtk::ListBox;
use gtk::ScrolledWindow;
use gtk::PolicyType;

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

    // Create a `ListBox` and add labels with integers from 0 to 100
    let list_box = ListBox::new();

    for number in 0..=30 {
        let label = Label::new(Some(&number.to_string()));
        list_box.append(&label);
    }

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(360)
        .child(&list_box)
        .build();

    let window = ApplicationWindow::builder()
        .application(application)
        .title("Counter")
        .default_width(500)
        .default_height(700)
        .child(&scrolled_window)
        .build();

    window.present();
}
