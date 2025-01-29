#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use egui::RichText;
use egui::Label;
use egui::Widget;
use egui::UiBuilder;
use egui::Sense;
use egui::Frame;
use egui::Color32;

fn main() -> eframe::Result {
    //env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {
    name: String,
    age: u32,
    count: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            count: 0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));

            ui.separator();

            ui.button("Quit").on_hover_ui(|ui| {
                ui.label("Click to quit the application");
            });
            
            //ui.image(egui::include_image!("../assets/sample.png"));
            //ui.image("file://assets/sample.png");
   
            ui.label("This demo demonstrates highlighting a widget.");
            ui.add_space(4.0);
            let label_response = ui.label("Hover me to highlight the button!");
            ui.add_space(4.0);
            let mut button_response = ui.button("Hover the button to highlight the label!");
    
            if label_response.hovered() {
                button_response = button_response.highlight();
            }
            if button_response.hovered() {
                label_response.highlight();
            }

            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label("This demo showcases how to use ");
                ui.code("Ui::response");
                ui.label(" to create interactive container widgets that may contain other widgets.");
            });
    
            let response = ui
                .scope_builder(
                    UiBuilder::new()
                        .id_salt("interactive_container")
                        .sense(Sense::click()),
                    |ui| {
                        let response = ui.response();
                        let visuals = ui.style().interact(&response);
                        let text_color = visuals.text_color();
    
                        Frame::canvas(ui.style())
                            .fill(visuals.bg_fill.gamma_multiply(0.3))
                            .stroke(visuals.bg_stroke)
                            .inner_margin(ui.spacing().menu_margin)
                            .show(ui, |ui| {
                                ui.set_width(ui.available_width());
    
                                ui.add_space(32.0);
                                ui.vertical_centered(|ui| {
                                    Label::new(
                                        RichText::new(format!("{}", self.count))
                                            .color(text_color)
                                            .size(32.0),
                                    )
                                    .selectable(false)
                                    .ui(ui);
                                });
                                ui.add_space(32.0);
    
                                ui.horizontal(|ui| {
                                    if ui.button("Reset").clicked() {
                                        self.count = 0;
                                    }
                                    if ui.button("+ 100").clicked() {
                                        self.count += 100;
                                    }
                                });
                            });
                    },
                )
                .response;
    
            if response.clicked() {
                self.count += 1;
            }

            ui.label(RichText::new("Text can have").color(Color32::from_rgb(110, 255, 110)));
            ui.colored_label(Color32::from_rgb(128, 140, 255), "color"); // Shortcut version
            ui.label("and tooltips.").on_hover_text(
                "This is a multiline tooltip that demonstrates that you can easily add tooltips to any element.\nThis is the second line.\nThis is the third.",
            );

            ui.label("You can mix in other widgets into text, like");
            let _ = ui.small_button("this button");
            ui.label(".");

            ui.label("The default font supports all latin and cyrillic characters (ИÅđ…), common math symbols (∫√∞²⅓…), and many emojis (💓🌟🖩…).")
            .on_hover_text("There is currently no support for right-to-left languages.");
        
            ui.label("See the 🔤 Font Book for more!");

            ui.monospace("There is also a monospace font.");

        });
    }
}