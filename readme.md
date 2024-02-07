# bmp8bit

Popular image libraries written in Rust do not allow color table based bmps to be saved. This small crate extends [image-rs](https://github.com/image-rs/image) so it is able to write `Dynamic Image` as 256-colors BMP. 

Theoretically it can write images with custom color tables, but I never tried it. See the Usage section for further details.

## Building

```
cargo build
```

## Usage
If you use image-rs, usage is just:

```
use bmp8bit::{self, save_8bit_win};
let img: image::DynamicImage = somehow_acquired_image();
save_8bit_win(img, "out/img_8bit.bmp");
```

### Your own raster image representation

If your have your own raster image representation implemented, you'll need to implement `Colors256` trait. It has two methods:
- `samples` is for converting your raster image representation to bytes vector using specified color table. This vector should contain the whole image. Use `ColorTableSample::new` to convert your sample to color table sample and then `ColorTable::convert` to acquire vector of bytes for your sample.
- `size` just returns `Dimensions` struct which describes your image width and height.

You can find reference implementation in [image_rs](https://github.com/zelenyhleb/bmp8bit/blob/master/src/image_rs.rs) module.

### Custom color table

Theoretically, `Colors256::samples` using custom color table would also generate a valid bmp image. Needs some testing. [windows_color_table](https://github.com/zelenyhleb/bmp8bit/blob/master/src/color_tables/windows_color_table.rs) can be taken as an example.

## License

Apache License Version 2.0

## Contributing

I am not really good in Rust yet, so any issue reports, improvements, suggestions (even for code style) are appreciated.