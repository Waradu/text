use eframe::egui;
use eframe::egui::ViewportBuilder;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file_content = String::from("No file specified.");

    if args.len() > 1 {
        let file_path = &args[1];
        file_content = match fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(_) => format!("Failed to read file: {}", file_path),
        };
    }

    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([300., 300.]),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Text Viewer",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp { file_content }))),
    );
}

struct MyApp {
    file_content: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(&self.file_content);
        });
    }
}
