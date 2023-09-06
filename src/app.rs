#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::io::Cursor;
use eframe::egui;
use egui_extras::RetainedImage;
use bmp_rust::bmp::BMP;
use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, RgbImage};

mod bmp_generator;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(400.0, 1000.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Show an image with eframe/egui",
        options,
        Box::new(|_cc| Box::<TestPatternGenerator>::default()),
    )
}


#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TestPatternGenerator {
    #[serde(skip)]
    image: RetainedImage,
    rounding: f32,
    tint: egui::Color32,
    #[serde(skip)]
    bmp: Option<BMP>,
}

impl Default for TestPatternGenerator {
    fn default() -> Self {
        let bmp_from_file = image::open("assets/test.bmp").unwrap();

        let mut bytes = Vec::new();
        bmp_from_file.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png).unwrap();

        Self {
            // crab image is CC0, found on https://stocksnap.io/search/crab
            //image: RetainedImage::from_image_bytes("crab.png", include_bytes!("/Users/andrew/Development/test_pattern_generator/assets/test_pattern.png")).unwrap(),
            image: RetainedImage::from_image_bytes("crab.png", bytes.as_slice()).unwrap(),
            rounding: 32.0,
            tint: egui::Color32::from_rgb(100, 200, 200),
            bmp: None,
        }
    }
}

impl TestPatternGenerator {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    pub fn update_image_with_bmp_file(&mut self, path: &str) {
        let bmp_from_file = image::open(path).unwrap();

        let mut bytes = Vec::new();
        bmp_from_file.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png).unwrap();

        self.image = RetainedImage::from_image_bytes("image.png", bytes.as_slice()).unwrap();
    }

    pub fn update_image_with_bmp(&mut self) {

        self.bmp = Some(bmp_generator::bmp_generator::BmpGenerator::generate_test());

        let bytes = &self.bmp.as_ref().unwrap().contents;

        self.image = RetainedImage::from_image_bytes("image.png", bytes.as_slice()).unwrap();
    }

    pub fn save_image(&self, path: &str) {
        self.bmp.clone().unwrap().save_to_new(path).unwrap();
    }
}

impl eframe::App for TestPatternGenerator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            if ui.button("Generate").clicked() {
                self.update_image_with_bmp();
            }

            if ui.button("Save").clicked() {
                self.save_image("assets/image.bmp");
            }

            ui.heading("This is an image:");
            self.image.show_scaled(ui, 500.0 / self.image.height() as f32);

            ui.add_space(32.0);

            /*ui.heading("This is a tinted image with rounded corners:");
            ui.add(
                egui::Image::new(self.image.texture_id(ctx), self.image.size_vec2())
                    .tint(self.tint)
            );

            ui.horizontal(|ui| {
                ui.label("Tint:");
                egui::color_picker::color_edit_button_srgba(
                    ui,
                    &mut self.tint,
                    egui::color_picker::Alpha::BlendOrAdditive,
                );

                ui.add_space(16.0);

                ui.label("Rounding:");
                ui.add(
                    egui::DragValue::new(&mut self.rounding)
                        .speed(1.0)
                        .clamp_range(0.0..=0.5 * self.image.size_vec2().min_elem()),
                );
            });

            ui.add_space(32.0);*/

            /*ui.heading("This is an image you can click:");
            ui.add(egui::ImageButton::new(
                self.image.texture_id(ctx),
                self.image.size_vec2(),
            ));*/
        });
    }
}