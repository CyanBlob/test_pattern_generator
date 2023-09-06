use bmp_rust::bmp::BMP;

pub struct BmpGenerator {}

impl BmpGenerator {
    pub fn generate_test() -> BMP {
        let mut bmp_from_scratch = BMP::new(
            2160,
            3840,
            Some([255, 255, 0, 255]),
        );

        //bmp_from_scratch.draw_ellipse([1920, 1080], 1000, 1000, [0, 255, 255, 255], Some([255, 0, 0, 255]), false).expect("Failed to bucket fill");
        bmp_from_scratch.draw_rectangle(Some([0, 255, 255, 255]), Some([255, 0, 0, 255]), [300, 300], [3340, 1860]).expect("Failed to bucket fill");

        bmp_from_scratch
    }
}