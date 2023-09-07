#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::cmp::min;
use bmp_rust::bmp::BMP;
use eframe::egui;
use egui_extras::RetainedImage;
use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, RgbImage};
use std::io::Cursor;

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
    #[serde(skip)]
    bmp: Option<BMP>,
    height: u16,
    width: u16,
    scale: f32,
    num_stripe_colors: u16,
    stripe_spacing: u16,
    rect_start: [u16; 2],
    rect_end: [u16; 2],
    rect_color: egui::Color32,
    elipse_center: [u16; 2],
    elipse_size: [u16; 2],
    elipse_color: egui::Color32,
}

impl Default for TestPatternGenerator {
    fn default() -> Self {
        let bmp_from_file = image::open("assets/test.bmp").unwrap();

        let mut bytes = Vec::new();
        bmp_from_file
            .write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)
            .unwrap();

        let bmp = Some(bmp_generator::bmp_generator::BmpGenerator::clear(
            1920,
            1080,
            None,
        ));

        let bytes = &bmp.as_ref().unwrap().contents;

        let image = RetainedImage::from_image_bytes("image.png", bytes.as_slice())
            .unwrap()
            .with_options(egui::TextureOptions::NEAREST);

        Self {
            image: image,
            rounding: 32.0,
            bmp: bmp,
            width: 1920,
            height: 1080,
            num_stripe_colors: 8,
            stripe_spacing: 1,
            scale: 500.0 / 1080.0,
            rect_start: [100, 100],
            rect_end: [200, 200],
            rect_color: egui::Color32::from_rgb(100, 200, 200),
            elipse_center: [1920 / 2, 1080 / 2],
            elipse_size: [200, 200],
            elipse_color: egui::Color32::from_rgb(200, 100, 100),
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
        bmp_from_file
            .write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)
            .unwrap();

        self.image = RetainedImage::from_image_bytes("image.png", bytes.as_slice()).unwrap();
    }

    pub fn update_image_with_bmp(&mut self) {
        self.bmp = Some(bmp_generator::bmp_generator::BmpGenerator::generate_test(
            &mut self.bmp.as_mut().unwrap(),
        ));

        self.update_image();
    }

    pub fn update_image_with_bmp_stripes(&mut self) {
        self.bmp = Some(
            bmp_generator::bmp_generator::BmpGenerator::generate_stripes(
                self.width,
                self.height,
                self.stripe_spacing,
                self.num_stripe_colors,
            ),
        );

        self.update_image();
    }

    pub fn add_rect(&mut self) {
        self.bmp = Some(bmp_generator::bmp_generator::BmpGenerator::add_rect(
            &mut self.bmp.as_mut().unwrap(),
            self.rect_start,
            self.rect_end,
            self.rect_color.to_array(),
        ));

        self.update_image();
    }

    pub fn add_elipse(&mut self) {
        self.bmp = Some(bmp_generator::bmp_generator::BmpGenerator::add_elipse(
            &mut self.bmp.as_mut().unwrap(),
            self.elipse_center,
            self.elipse_size,
            self.elipse_color.to_array(),
        ));

        self.update_image();
    }


    pub fn update_image(&mut self) {
        let bytes = &self.bmp.as_ref().unwrap().contents;

        self.image = RetainedImage::from_image_bytes("image.png", bytes.as_slice())
            .unwrap()
            .with_options(egui::TextureOptions::NEAREST)
    }

    pub fn save_image(&self, path: &str) {
        self.bmp.clone().unwrap().save_to_new(path).unwrap();
    }
}

impl eframe::App for TestPatternGenerator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(egui::Slider::new(&mut self.width, 0..=3840 * 2).text("Width"));
            ui.add(egui::Slider::new(&mut self.height, 0..=2160 * 2).text("Height"));

            if ui.button("Reset").clicked() {
                self.bmp = Some(bmp_generator::bmp_generator::BmpGenerator::clear(
                    self.width,
                    self.height,
                    None,
                ));
                self.update_image();
            }

            ui.add_space(32.0);

            ui.add(egui::Slider::new(&mut self.rect_start[0], 0..=self.width).text("Start X"));
            ui.add(egui::Slider::new(&mut self.rect_start[1], 0..=self.height).text("Start Y"));

            ui.add(egui::Slider::new(&mut self.rect_end[0], 0..=self.width).text("End X"));
            ui.add(egui::Slider::new(&mut self.rect_end[1], 0..=self.height).text("End Y"));

            ui.label("Color:");
            egui::color_picker::color_edit_button_srgba(
                ui,
                &mut self.rect_color,
                egui::color_picker::Alpha::BlendOrAdditive,
            );

            if ui.button("Generate rect").clicked() {
                self.add_rect();
            }

            ui.add(egui::Slider::new(&mut self.elipse_center[0], 0..=self.width).text("Center X"));
            ui.add(egui::Slider::new(&mut self.elipse_center[1], 0..=self.height).text("Center Y"));

            ui.add(egui::Slider::new(&mut self.elipse_size[0], 0..=self.width).text("Width"));
            ui.add(egui::Slider::new(&mut self.elipse_size[1], 0..=self.height).text("Height"));

            ui.label("Color:");
            egui::color_picker::color_edit_button_srgba(
                ui,
                &mut self.elipse_color,
                egui::color_picker::Alpha::BlendOrAdditive,
            );

            if ui.button("Generate elipse").clicked() {
                self.add_elipse();
            }

            ui.add_space(32.0);

            ui.add(egui::Slider::new(&mut self.num_stripe_colors, 0..=8).text("Num Colors"));
            ui.add(
                egui::Slider::new(&mut self.stripe_spacing, 0..=2160)
                    .step_by(1.0)
                    .text("Spacing"),
            );

            if ui.button("Generate stripes").clicked() {
                self.update_image_with_bmp_stripes();
            }

            if ui.button("Save").clicked() {
                self.save_image("assets/image.bmp");
            }
        });
        egui::SidePanel::right("Image panel")
            .max_width(1500.0)
            .show(ctx, |ui| {
                ui.heading("Preview:");
                ui.add(
                    egui::Slider::new(&mut self.scale, 0.1..=20.0)
                        .step_by(0.01)
                        .text("Scale"),
                );
                egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                    egui::ScrollArea::horizontal().show(ui, |ui| {
                        self.image.show_scaled(ui, self.scale);
                    });
                });
            });
    }
}
