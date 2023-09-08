use std::cmp::{max, min};
use std::f64::consts::PI;
use bmp::{Image, Pixel};

pub struct BmpGenerator {}

pub fn point_in_ellipse(h: f64, k: f64, a: f64, b: f64, x: f64, y: f64) -> f64 {
    let p = (((x - h) * (x - h)) / (a * a)) + (((y - k) * (y - k)) / (b * b));

    p
}

pub fn rotated_rect_contains(rect_start: [u32; 2], rect_end: [u32; 2], rect_angle: f64, point: [u32; 2]) -> bool
{
    let angle = rect_angle * PI / 180.0;

    let width = (rect_end[0] - rect_start[0]) as f64;
    let height = (rect_end[1] - rect_start[1]) as f64;

    let center = [(rect_end[0] - rect_start[0]) as f64 / 2.0 + rect_start[0] as f64, (rect_end[1] - rect_start[1]) as f64 / 2.0 + rect_start[1] as f64];

    // un-rotate the point
    let cos = angle.cos();
    let sin = angle.sin();

    let x = (point[0] as f64 - center[0]) * cos + (point[1] as f64 - center[1]) * sin;
    let y = -(point[0] as f64 - center[0]) * sin + (point[1] as f64 - center[1]) * cos;

    // test if the un-rotated point is inside the un-rotated rectangle
    x >= (width * -1.0) / 2.0 && x <= width / 2.0 && y >= (height * -1.0) / 2.0 && y <= height / 2.0
}

fn rotated_ellipse_contains(h: f64, k: f64, a: f64, b: f64, angle: f64, pixel: (f64, f64)) -> f64 {
    let angle = angle * PI / 180.0;
    // from: https://stackoverflow.com/questions/7946187/point-and-ellipse-rotated-position-test-algorithm
    (((h - pixel.0) * angle.cos() + (k - pixel.1) * angle.sin()) / a) * (((h - pixel.0) * angle.cos() + (k - pixel.1) * angle.sin()) / a)
        + (((h - pixel.0) * angle.sin() - (k - pixel.1) * angle.cos()) / b) * (((h - pixel.0) * angle.sin() - (k - pixel.1) * angle.cos()) / b)
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
        rotation: f64,
    ) -> Image {
        let width = bmp.get_width();
        let height = bmp.get_height();

        let rect_width = rect_end[0] - rect_start[0];
        let rect_height = rect_end[1] - rect_start[1];

        let mut start_x = max(0, rect_start[0] as i32);
        let mut start_y = max(0, rect_start[1] as i32);
        let mut end_x = min(width, rect_end[0]);
        let mut end_y = min(height, rect_end[1]);

        if rotation != 0.0 {
            start_x = max(0, ((rect_start[0] as i32 + rect_width as i32 / 2) - rect_width as i32));
            start_y = max(0, ((rect_start[1] as i32 + rect_height as i32 / 2) - rect_height as i32));
            end_x = min(width, (rect_width) + (rect_start[0] + rect_width / 2));
            end_y = min(height, (rect_height) + (rect_start[1] + rect_height / 2));
        }

        for x in start_x as u32..end_x {
            for y in start_y as u32..end_y {
                if rotation != 0.0 {
                    if rotated_rect_contains(rect_start, rect_end, rotation, [x, y]) {
                        let _ = bmp.set_pixel(
                            x,
                            y,
                            Pixel::new(rect_color[0], rect_color[1], rect_color[2]),
                        );
                    }
                } else {
                    let _ = bmp.set_pixel(
                        x,
                        y,
                        Pixel::new(rect_color[0], rect_color[1], rect_color[2]),
                    );
                }
            }
        }

        bmp.clone()
    }

    pub fn add_ellipse(
        bmp: &mut Image,
        center: [i32; 2],
        size: [i32; 2],
        rotation: f64,
        ellipse_color: [u8; 3],
    ) -> Image {
        let width = bmp.get_width() as i32;
        let height = bmp.get_height() as i32;

        // size +1 to account for rounding issues after halving
        let bounding_box_size;

        if rotation != 0.0 {
            bounding_box_size = match size[0] < size[1] {
                true => {
                    [size[1] + 1, size[1] + 1]
                }
                false => {
                    [size[0] + 1, size[0] + 1]
                }
            };
        } else {
            bounding_box_size = [size[0] + 1, size[1] + 1];
        }

        bounding_box_size.map(|s| { s / 2 });

        let ellipse_start_x = center[0] - size[0];
        let ellipse_end_x = center[0] + size[0];

        let ellipse_start_y = center[1] - size[1];
        let ellipse_end_y = center[1] + size[1];

        let mut start_x = max(0, ellipse_start_x);
        let mut start_y = max(0, ellipse_start_y);
        let mut end_x = min(width, ellipse_end_x);
        let mut end_y = min(height, ellipse_end_y);

        if rotation != 0.0 {
            start_x = max(0, center[0] - bounding_box_size[0]);
            start_y = max(0, center[1] - bounding_box_size[1]);
            end_x = min(width, center[0] + bounding_box_size[0]);
            end_y = min(height, center[1] + bounding_box_size[1]);
        }
        for x in start_x..end_x {
            for y in start_y..end_y {
                match rotated_ellipse_contains(
                    center[0] as f64,
                    center[1] as f64,
                    size[0] as f64,
                    size[1] as f64,
                    rotation,
                    (x as f64,
                     y as f64),
                ) {
                    n if n <= 1.0 => {
                        let _ = bmp.set_pixel(
                            x as u32,
                            y as u32,
                            Pixel::new(ellipse_color[0], ellipse_color[1], ellipse_color[2]),
                        );
                    }
                    _ => {
                        // draw bounding box
                        //let _ = bmp.set_pixel( x as u32, y as u32, Pixel::new(255, 0, 0), );
                    }
                }
            }
        }

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