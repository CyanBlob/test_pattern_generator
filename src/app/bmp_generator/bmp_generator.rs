use bmp::{Image, Pixel};

pub struct BmpGenerator {}

pub fn point_in_elipse(h: f64, k: f64, a: f64, b: f64, x: f64, y: f64) -> f64 {
    let p = (((x - h) * (x - h)) / (a * a)) + (((y - k) * (y - k)) / (b * b));

    p
}

impl BmpGenerator {
    pub fn clear(width: u32, height: u32) -> Image {
        let bmp = Image::new(width, height);

        bmp
    }

    pub fn add_rect(
        bmp: &mut Image,
        rect_start: [u32; 2],
        rect_end: [u32; 2],
        rect_color: [u8; 3],
    ) -> Image {
        let width = bmp.get_width();
        let height = bmp.get_height();

        let start_x = rect_start[0];

        let end_x;
        if rect_end[0] > width {
            end_x = width - 1;
        } else {
            end_x = rect_end[0];
        }

        let start_y = rect_start[1];

        let end_y;
        if rect_end[1] > height {
            end_y = height - 1;
        } else {
            end_y = rect_end[1];
        }

        for x in start_x..end_x {
            for y in start_y..end_y {
                let _ = bmp.set_pixel(
                    x,
                    y,
                    Pixel::new(rect_color[0], rect_color[1], rect_color[2]),
                );
            }
        }

        bmp.clone()
    }

    pub fn add_elipse(
        bmp: &mut Image,
        center: [u32; 2],
        size: [u32; 2],
        elipse_color: [u8; 3],
    ) -> Image {
        let width = bmp.get_width();
        let height = bmp.get_height();

        let start_x;
        if center[0] < size[0] {
            start_x = 0;
        } else {
            start_x = center[0] - size[0];
        }

        for x in start_x..center[0] + size[0] {
            if x >= width {
                continue;
            }

            let start_y;
            if center[1] < size[1] {
                start_y = 0;
            } else {
                start_y = center[1] - size[1];
            }

            for y in start_y..center[1] + size[1] {
                if y >= height {
                    continue;
                }

                match point_in_elipse(
                    center[0] as f64,
                    center[1] as f64,
                    size[0] as f64,
                    size[1] as f64,
                    x as f64,
                    y as f64,
                ) {
                    n if n <= 1.0 => {
                        let _ = bmp.set_pixel(
                            x,
                            y,
                            Pixel::new(elipse_color[0], elipse_color[1], elipse_color[2]),
                        );
                    }
                    _ => {}
                }
            }
        }

        // for drawing a good border
        //let _ = bmp.draw_ellipse(center, size[0], size[1], [0, 255, 255, 255], None, true);

        bmp.clone()
    }

    pub fn generate_stripes(width: u32, height: u32, spacing: u32, num_colors: u32) -> Image {
        let mut bmp = Image::new(width, height);

        let mut color_index = 0;

        for x in (0..width).step_by(spacing as usize) {
            let color = match (color_index) % num_colors {
                0 => [255, 0, 0, 255],
                1 => [0, 255, 0, 255],
                2 => [0, 0, 255, 255],
                3 => [255, 255, 0, 255],
                4 => [0, 255, 255, 255],
                5 => [255, 0, 255, 255],
                6 => [0, 0, 0, 255],
                7 => [255, 255, 255, 255],
                _ => [0, 0, 0, 255],
            };

            color_index = color_index + 1;

            for i in 0..spacing {
                for j in 0..height {
                    if i + x < width {
                        let _ = bmp.set_pixel(i + x, j, Pixel::new(color[0], color[1], color[2]));
                    }
                }
            }
        }

        bmp
    }
}
