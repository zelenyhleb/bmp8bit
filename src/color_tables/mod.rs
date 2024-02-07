pub mod windows_color_table;

use std::collections::HashMap;

use itertools::Itertools;
#[derive(Clone, Copy, Hash, Eq)]
pub struct ColorTableSample {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl PartialEq for ColorTableSample {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red && self.green == other.green && self.blue == other.blue
    }
}

impl ColorTableSample {
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        ColorTableSample {
            red: red,
            green: green,
            blue: blue,
            alpha: alpha,
        }
    }

    pub fn white() -> Self {
        ColorTableSample::new(255, 255, 255, 255)
    }

    pub fn black() -> Self {
        ColorTableSample::new(0, 0, 0, 255)
    }

    fn append_self(&self, to: &mut Vec<u8>) {
        // I don't know why, but exactly this order gives a valid 256-color bmp.
        to.push(self.blue);
        to.push(self.green);
        to.push(self.red);
        to.push(0x00);
    }
}

pub struct ColorTable {
    direct: HashMap<ColorTableSample, u8>,
    derived: HashMap<ColorTableSample, u8>,
}

impl ColorTable {
    pub fn new(content: HashMap<ColorTableSample, u8>) -> Self {
        ColorTable {
            direct: content,
            derived: HashMap::new(),
        }
    }

    pub fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        self.direct
            .keys()
            .sorted_by(|k1, k2| {
                (*self.direct.get(k1).unwrap()).cmp(&(*self.direct.get(k2).unwrap()))
            })
            .for_each(|sample| sample.append_self(&mut bytes));
        bytes
    }

    pub fn convert(&mut self, sample: ColorTableSample) -> u8 {
        if self.direct.contains_key(&sample) {
            return *self.direct.get(&sample).unwrap();
        }
        *self
            .derived
            .entry(sample)
            .or_insert_with(|| ColorTable::nearest(&self.direct, sample))
    }

    fn nearest(table: &HashMap<ColorTableSample, u8>, sample: ColorTableSample) -> u8 {
        *table
            .keys()
            .min_by(|c1, c2| {
                ColorTable::difference(c1, &sample).total_cmp(&ColorTable::difference(c2, &sample))
            })
            .and_then(|key| table.get(key))
            .unwrap_or(&0xff)
    }

    fn difference(candidate: &ColorTableSample, sample: &ColorTableSample) -> f32 {
        let red = candidate.red.abs_diff(sample.red) as f32;
        let green = candidate.green.abs_diff(sample.green) as f32;
        let blue = candidate.blue.abs_diff(sample.blue) as f32;
        ((0.3 * red).powi(2) + (0.59 * green).powi(2) + (0.11 * blue).powi(2)).sqrt()
    }
}
