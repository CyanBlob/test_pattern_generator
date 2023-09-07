use bmp_rust::bmp::BMP;

pub struct BmpGenerator {}

pub fn point_in_elipse(h: f64, k: f64, a: f64, b: f64, x: f64, y: f64) -> f64
{
    let p = (((x - h) * (x - h)) / (a * a))
        + (((y - k) * (y - k)) / (b * b));

    p
}

impl BmpGenerator {
    pub fn clear(width: u16, height: u16, color: Option<[u8; 4]>) -> BMP {
        let mut bmp = BMP::new(height as i32, width as u32, color);

        bmp
    }

    pub fn add_rect(bmp: &mut BMP, rect_start: [u16; 2], rect_end: [u16; 2], rect_color: [u8; 4]) -> BMP {
        let width = bmp.get_dib_header().unwrap().width as u16;
        let height = bmp.get_dib_header().unwrap().height as u16;

        let start_x;
        if rect_start[0] < 0 {
            start_x = 0;
        } else {
            start_x = rect_start[0];
        }

        let end_x;
        if rect_end[0] > width {
            end_x = width - 1;
        } else {
            end_x = rect_end[0];
        }

        let start_y;
        if rect_start[1] < 0 {
            start_y = 0;
        } else {
            start_y = rect_start[1];
        }

        let end_y;
        if rect_end[1] > height {
            end_y = height - 1;
        } else {
            end_y = rect_end[1];
        }

        for x in start_x..end_x {
            for y in start_y..end_y {
                let _ = bmp.change_color_of_pixel(x, y, rect_color);
            }
        }

        bmp.clone()
    }

    pub fn add_elipse(bmp: &mut BMP, center: [u16; 2], size: [u16; 2], elipse_color: [u8; 4]) -> BMP {
        let width = bmp.get_dib_header().unwrap().width;
        let height = bmp.get_dib_header().unwrap().height;

        let start_x;
        if center[0] < size[0] {
            start_x = 0;
        } else {
            start_x = center[0] - size[0];
        }

        for x in start_x..center[0] + size[0] {
            if x < 0 || x >= width as u16 {
                continue;
            }

            let start_y;
            if center[1] < size[1] {
                start_y = 0;
            } else {
                start_y = center[1] - size[1];
            }

            for y in start_y..center[1] + size[1] {
                if y < 0 || y >= height as u16 {
                    continue;
                }

                match point_in_elipse(center[0] as f64, center[1] as f64, size[0] as f64, size[1] as f64, x as f64, y as f64) {
                    n if n <= 1.0 => {
                        let _ = bmp.change_color_of_pixel(x, y, elipse_color);
                    }
                    _ => {}
                }
            }
        }

        // for drawing a good border
        //let _ = bmp.draw_ellipse(center, size[0], size[1], [0, 255, 255, 255], None, true);

        bmp.clone()
    }


    pub fn generate_test(bmp: &mut BMP) -> BMP {
        let mut bmp_from_scratch = BMP::new(2160, 3840, Some([255, 255, 0, 255]));

        //bmp_from_scratch.draw_ellipse([1920, 1080], 1000, 1000, [0, 255, 255, 255], Some([255, 0, 0, 255]), false).expect("Failed to bucket fill");
        bmp.draw_rectangle(
            Some([0, 255, 255, 255]),
            Some([255, 0, 0, 255]),
            [300, 300],
            [600, 600],
        )
            .expect("Failed to bucket fill");

        bmp.clone()
    }

    pub fn generate_stripes(width: u16, height: u16, spacing: u16, num_colors: u16) -> BMP {
        let mut bmp = BMP::new(height as i32, width as u32, Some([255, 255, 0, 255]));

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

            for i in 0..spacing as u16 {
                for j in 0..height {
                    if i + x < width
                    {
                        let _ = bmp.change_color_of_pixel(i + x, j, color);
                    }
                }
            }
        }

        bmp
    }
}
