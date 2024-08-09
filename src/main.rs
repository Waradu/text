#![windows_subsystem = "windows"]

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
        viewport: ViewportBuilder::default()
            .with_inner_size([700., 600.])
            .with_min_inner_size([400., 200.]),
        centered: true,
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Text Viewer",
        options,
        Box::new(|_cc| Ok(Box::new(TextApp { file_content }))),
    );
}

struct TextApp {
    file_content: String,
}

impl eframe::App for TextApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            std::process::exit(0);
        }

        let mut style: egui::Style = (*ctx.style()).clone();
        style
            .text_styles
            .get_mut(&egui::TextStyle::Body)
            .unwrap()
            .size = 18.0;
        ctx.set_style(style);

        egui::CentralPanel::default().show(ctx, |ui| {
            let available_width = ui.available_width();

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.set_min_width(available_width);
                ui.colored_label(egui::Color32::WHITE, &self.file_content);
            });
        });
    }
}
