use bmp_rust::bmp::BMP;

pub struct BmpGenerator {}

impl BmpGenerator {
    pub fn clear(width: u16, height: u16, color: Option<[u8; 4]>) -> BMP {
        let mut bmp = BMP::new(height as i32, width as u32, color);

        bmp
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
                    let _ = bmp.change_color_of_pixel(i + x, j, color);
                }
            }
        }

        bmp
    }
}
