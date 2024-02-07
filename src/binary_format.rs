use crate::{color_tables::ColorTable, Colors256};

const SIZE_FILE_HEADER: u32 = 14;
const SIZE_DIB_HEADER: u32 = 40;

pub fn raw(image: &impl Colors256, color_table: &mut ColorTable) -> Vec<u8> {
    let mut bytes = vec![];
    /* File header */
    bytes.append(&mut vec![0x42, 0x4D]); // Magic number
    bytes.append(&mut size(image, &color_table)); // BMP file size
    bytes.append(&mut vec![0x00; 4]); // Reserved bytes. Four zeros
    bytes.append(&mut offset(&color_table)); // Pixels array offset

    /* BITMAPINFOHEADER DIB header.*/
    bytes.append(&mut vec_of_bytes(SIZE_DIB_HEADER)); // Size of header in bytes
    bytes.append(&mut vec_of_bytes(image.dimensions().width)); // Image wigth
    bytes.append(&mut vec_of_bytes(image.dimensions().height)); // Image height
    bytes.append(&mut vec![0x01, 0x00]); // One color plane
    bytes.append(&mut vec![0x08, 0x00]); // Number of bits per pixel = 8
    bytes.append(&mut compression_method()); // Compression method
    bytes.append(&mut vec_of_bytes(0_u32)); // Samples array size, which is 0 for no compression
    bytes.append(&mut vec_of_bytes(0_u32)); // Print resolution horizontal
    bytes.append(&mut vec_of_bytes(0_u32)); // Print resolution vertical
    bytes.append(&mut vec_of_bytes(0_u32)); // Number of colors in palette (0 is default and means 2^n colors for n-bits color depth)
    bytes.append(&mut vec_of_bytes(0_u32)); // Important colors, 0 for "all colors are important"

    /* Color table */
    bytes.append(&mut color_table.bytes()); // Using windows color table for now. We will probably improve it later

    /* Samples array */
    bytes.append(&mut image.samples(color_table));

    /* Finally return */
    bytes
}

fn size(image: &impl Colors256, table: &ColorTable) -> Vec<u8> {
    vec_of_bytes(
        SIZE_FILE_HEADER + SIZE_DIB_HEADER + table.bytes().len() as u32 + image.dimensions().size(),
    )
}

fn offset(table: &ColorTable) -> Vec<u8> {
    vec_of_bytes(SIZE_FILE_HEADER + SIZE_DIB_HEADER + table.bytes().len() as u32)
}

fn compression_method() -> Vec<u8> {
    /* 0 for no compression. This is the only way to feed this image to eclipse */
    vec_of_bytes(0_u32)
}

fn vec_of_bytes(number: u32) -> Vec<u8> {
    let mut bytes = vec![];
    bytes.extend(number.to_le_bytes());
    bytes
}

#[cfg(test)]
mod tests {
    use crate::binary_format::vec_of_bytes;

    #[test]
    fn bytes_conversion() {
        assert_eq!(vec![0x01, 0x00, 0x00, 0x00], vec_of_bytes(1u32));
        assert_eq!(vec![0x01, 0x01, 0x00, 0x00], vec_of_bytes(257u32));
    }
}
