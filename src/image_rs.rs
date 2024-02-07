use image::{DynamicImage, GenericImageView, Rgba};

use crate::{
    color_tables::{ColorTable, ColorTableSample},
    Colors256, Dimensions,
};

impl Colors256 for DynamicImage {
    fn samples(&self, colors: &mut ColorTable) -> Vec<u8> {
        self.flipv()
            .as_rgba8()
            .expect("Failed to convert image to RGBA: ")
            .enumerate_pixels()
            .into_iter()
            .map(|(_, _, sample)| colors.convert(ColorTableSample::from_sample(sample)))
            .collect()
    }

    fn dimensions(&self) -> Dimensions {
        Dimensions::from_tuple(GenericImageView::dimensions(self))
    }
}

impl ColorTableSample {
    pub fn from_sample(sample: &Rgba<u8>) -> Self {
        if nearly_invisible(sample) {
            return ColorTableSample::white();
        }
        ColorTableSample::new(sample.0[0], sample.0[1], sample.0[2], sample.0[3])
    }
}

fn nearly_invisible(sample: &Rgba<u8>) -> bool {
    sample.0[3] < 64
}
