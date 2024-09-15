use eframe::egui;
use std::fs::File;
use std::io::{Read, Write};

// Define a custom error type if necessary, or use a common error type like Box<dyn std::error::Error>
type DynError = Box<dyn std::error::Error>;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Simple Notepad",
        options,
        Box::new(|_cc| {
            // Return the application boxed in a Result
            Ok(Box::new(MyApp::default()) as Box<dyn eframe::App>)
        }),
    )
}

#[derive(Default)]
struct MyApp {
    text: String,
    file_path: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Simple Notepad");

            // Text input for the file path
            ui.horizontal(|ui| {
                ui.label("File Path:");
                ui.text_edit_singleline(&mut self.file_path);
            });

            // Multiline text area for the notepad content
            ui.add(
                egui::TextEdit::multiline(&mut self.text)
                    .hint_text("Type your notes here...")
                    .desired_rows(30),

            );

            // Buttons for saving and loading files
            if ui.button("Save").clicked() {
                if !self.file_path.is_empty() {
                    if let Err(err) = save_to_file(&self.file_path, &self.text) {
                        ui.label(format!("Error saving file: {}", err));
                    }
                }
            }

            // if ui.button("Load").clicked() {
            //     if !self.file_path.is_empty() {
            //         match load_from_file(&self.file_path) {
            //             Ok(content) => self.text = content,
            //             Err(err) => {
            //                 // Explicitly ignore the Response using let _
            //                 let _ = ui.label(format!("Error loading file: {}", err));
            //             }
            //         }
            //     }
            // }
        });
    }
}

fn save_to_file(path: &str, content: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

// fn load_from_file(path: &str) -> std::io::Result<String> {
//     let mut file = File::open(path)?;
//     let mut content = String::new();
//     file.read_to_string(&mut content)?;
//     Ok(content)
// }
