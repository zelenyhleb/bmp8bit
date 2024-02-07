use bmp8bit::{self, bytes_8bit, color_tables::windows_color_table::windows_color_table};

#[test]
fn write_multicolor_squares() {
    assert_eq!(
        std::fs::read("resources/squares_8bit.bmp").expect("Can't read image file"),
        bytes_8bit(
            image::open("resources/squares.png").expect("incorrect workspace configuration"),
            &mut windows_color_table()
        )
    );
}

#[test]
fn write_multicolor_lines() {
    assert_eq!(
        std::fs::read("resources/lines_8bit.bmp").expect("Can't read image file"),
        bytes_8bit(
            image::open("resources/lines.png").expect("incorrect workspace configuration"),
            &mut windows_color_table()
        )
    );
}
