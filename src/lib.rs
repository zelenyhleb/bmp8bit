pub mod binary_format;
pub mod color_tables;
pub mod image_rs;

use std::{fs::File, io::Write, path::Path, u32};

use color_tables::{windows_color_table::windows_color_table, ColorTable};

pub trait Colors256 {
    fn samples(&self, colors: &mut ColorTable) -> Vec<u8>;
    fn dimensions(&self) -> Dimensions;
}

pub struct Dimensions {
    pub width: u32,
    pub height: u32,
}

impl Dimensions {
    pub fn new(width: u32, height: u32) -> Self {
        Dimensions {
            width: width,
            height: height,
        }
    }

    pub fn from_tuple(dimensions: (u32, u32)) -> Self {
        Dimensions::new(dimensions.0, dimensions.1)
    }

    pub fn size(&self) -> u32 {
        self.width * self.height
    }
}

pub fn save_8bit_win<Q>(image: impl Colors256, path: Q) -> Result<(), std::io::Error>
where
    Q: AsRef<Path>,
{
    save_8bit(image, path, &mut windows_color_table())
}

pub fn save_8bit<Q>(
    image: impl Colors256,
    path: Q,
    colors: &mut ColorTable,
) -> Result<(), std::io::Error>
where
    Q: AsRef<Path>,
{
    File::create(path)?.write_all(&bytes_8bit(image, colors))
}

pub fn bytes_8bit(image: impl Colors256, colors: &mut ColorTable) -> Vec<u8> {
    binary_format::raw(&image, colors)
}
