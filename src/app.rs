//#![cfg_attr(not(debug_assertions))] // hide console window on Windows in release

use bmp::Image;
use eframe::egui;
use egui_extras::RetainedImage;
use std::io::Cursor;

mod bmp_generator;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TestPatternGenerator {
    #[serde(skip)]
    image: RetainedImage,
    rounding: f32,
    #[serde(skip)]
    bmp: Option<Image>,
    height: u32,
    width: u32,
    scale: f32,
    num_stripe_colors: u32,
    stripe_spacing: u32,
    rect_start: [u32; 2],
    rect_end: [u32; 2],
    rect_rotation: f64,
    rect_color: egui::Color32,
    ellipse_center: [u32; 2],
    ellipse_size: [u32; 2],
    ellipse_color: egui::Color32,
    ellipse_rotation: f64,
}

impl Default for TestPatternGenerator {
    fn default() -> Self {
        let bmp_from_file = image::open("assets/test.bmp").unwrap();

        let mut bytes = Vec::new();
        bmp_from_file
            .write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)
            .unwrap();

        let bmp = Some(bmp_generator::bmp_generator::BmpGenerator::clear(
            1920, 1080,
        ));

        let mut bytes: Vec<u8> = vec![];
        let _ = bmp.as_ref().unwrap().to_writer(&mut bytes);

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
            rect_start: [760, 340],
            rect_end: [1160, 740],
            rect_rotation: 0.0,
            rect_color: egui::Color32::from_rgb(255, 0, 255),
            ellipse_center: [1920 / 2, 1080 / 2],
            ellipse_size: [200, 200],
            ellipse_color: egui::Color32::from_rgb(0, 255, 255),
            ellipse_rotation: 0.0,
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
        let color = [
            self.rect_color.r(),
            self.rect_color.g(),
            self.rect_color.b(),
        ];
        self.bmp = Some(bmp_generator::bmp_generator::BmpGenerator::add_rect(
            &mut self.bmp.as_mut().unwrap(),
            self.rect_start,
            self.rect_end,
            color,
            self.rect_rotation,
        ));

        self.update_image();
    }

    pub fn add_ellipse(&mut self) {
        let color = [
            self.ellipse_color.r(),
            self.ellipse_color.g(),
            self.ellipse_color.b(),
        ];
        self.bmp = Some(bmp_generator::bmp_generator::BmpGenerator::add_ellipse(
            &mut self.bmp.as_mut().unwrap(),
            self.ellipse_center.map(|c| { c as i32 }),
            self.ellipse_size.map(|s| { s as i32 }),
            self.ellipse_rotation,
            color,
        ));

        self.update_image();
    }

    pub fn update_image(&mut self) {
        //let bytes = &self.bmp.as_ref().unwrap().contents;
        let mut bytes: Vec<u8> = vec![];
        let _ = self.bmp.as_ref().unwrap().to_writer(&mut bytes);

        self.image = RetainedImage::from_image_bytes("image.png", bytes.as_slice())
            .unwrap()
            .with_options(egui::TextureOptions::NEAREST)
    }

    pub fn save_image(&self, path: &str) {
        let bmp = self.bmp.clone().unwrap();
        bmp.save(path).unwrap();
    }
}

impl eframe::App for TestPatternGenerator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(egui::Slider::new(&mut self.width, 0..=3840 * 2).text("Width"));
            ui.add(egui::Slider::new(&mut self.height, 0..=2160 * 2).text("Height"));

            ui.add_space(5.0);

            if ui.button("Reset").clicked() {
                self.bmp = Some(bmp_generator::bmp_generator::BmpGenerator::clear(
                    self.width,
                    self.height,
                ));
                self.update_image();
            }

            ui.add_space(32.0);

            ui.add(egui::Slider::new(&mut self.rect_start[0], 0..=self.width).text("Start X"));
            ui.add(egui::Slider::new(&mut self.rect_start[1], 0..=self.height).text("Start Y"));

            ui.add(egui::Slider::new(&mut self.rect_end[0], 0..=self.width * 2).text("End X"));
            ui.add(egui::Slider::new(&mut self.rect_end[1], 0..=self.height * 2).text("End Y"));

            ui.add_space(10.0);

            ui.add(egui::Slider::new(&mut self.rect_rotation, 0.0..=360.0).text("Rotation"));

            //ui.label("Color:");
            ui.add_space(10.0);
            ui.add(
                egui::Slider::new(&mut self.rect_color[0], 0..=255)
                    .text("Red")
                    .text_color(egui::Color32::from_rgb(255, 0, 0)),
            );
            ui.add(
                egui::Slider::new(&mut self.rect_color[1], 0..=255)
                    .text("Green")
                    .text_color(egui::Color32::from_rgb(0, 255, 0)),
            );
            ui.add(
                egui::Slider::new(&mut self.rect_color[2], 0..=255)
                    .text("Blue")
                    .text_color(egui::Color32::from_rgb(0, 0, 255)),
            );
            egui::color_picker::color_edit_button_srgba(
                ui,
                &mut self.rect_color,
                egui::color_picker::Alpha::Opaque,
            );

            ui.add_space(5.0);

            ui.horizontal(|ui| {
                if ui.button("Generate rect").clicked() {
                    self.add_rect();
                }

                if ui.button("Rotation sweep").clicked() {
                    let original_rotation = self.rect_rotation;
                    for i in (0..360).step_by(self.rect_rotation as usize)
                    {
                        self.rect_rotation = i as f64;
                        self.add_rect();
                    }
                    self.rect_rotation = original_rotation;
                }
            });

            ui.add_space(32.0);

            ui.add(egui::Slider::new(&mut self.ellipse_center[0], 0..=self.width).text("Center X"));
            ui.add(egui::Slider::new(&mut self.ellipse_center[1], 0..=self.height).text("Center Y"));

            ui.add(egui::Slider::new(&mut self.ellipse_size[0], 0..=self.width).text("Radius X"));
            ui.add(egui::Slider::new(&mut self.ellipse_size[1], 0..=self.height).text("Radius Y"));

            ui.add_space(10.0);

            ui.add(egui::Slider::new(&mut self.ellipse_rotation, 0.0..=360.0).text("Rotation"));

            //ui.label("Color:");
            ui.add_space(10.0);
            ui.add(
                egui::Slider::new(&mut self.ellipse_color[0], 0..=255)
                    .text("Red")
                    .text_color(egui::Color32::from_rgb(255, 0, 0)),
            );
            ui.add(
                egui::Slider::new(&mut self.ellipse_color[1], 0..=255)
                    .text("Green")
                    .text_color(egui::Color32::from_rgb(0, 255, 0)),
            );
            ui.add(
                egui::Slider::new(&mut self.ellipse_color[2], 0..=255)
                    .text("Blue")
                    .text_color(egui::Color32::from_rgb(0, 0, 255)),
            );
            egui::color_picker::color_edit_button_srgba(
                ui,
                &mut self.ellipse_color,
                egui::color_picker::Alpha::Opaque,
            );

            ui.add_space(5.0);

            if ui.button("Generate ellipse").clicked() {
                self.add_ellipse();
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
            .max_width(3840.0)
            .show(ctx, |ui| {
                ui.heading("Preview:");
                ui.add(
                    egui::Slider::new(&mut self.scale, 0.1..=20.0)
                        .step_by(0.01)
                        .text("Scale"),
                );
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        egui::ScrollArea::horizontal().show(ui, |ui| {
                            self.image.show_scaled(ui, self.scale);
                        });
                    });
            });
    }
}
