use std::path::Path;

use image::imageops::resize;
use image::imageops::FilterType;
use image::io::Reader;
use image::GenericImageView;
use image::ImageBuffer;
use image::ImageFormat;
use image::Rgba;
use image::RgbaImage;

use crate::consts::ICON_FILE_NAME;
use crate::fs::set_attr_hidden;

type SquaredPng = ImageBuffer<Rgba<u8>, Vec<u8>>;

fn sqr_png(path: impl AsRef<Path>) -> anyhow::Result<SquaredPng> {
    let img = Reader::open(path)?.decode()?;

    let (w, h) = img.dimensions();
    let size = w.max(h);

    let mut ret = RgbaImage::new(size, size);
    let x_s = (size - w) / 2;
    let y_s = (size - h) / 2;

    for x in 0..w {
        for y in 0..h {
            let p = img.get_pixel(x, y);
            ret.put_pixel(x + x_s, y + y_s, p);
        }
    }

    let x256 = resize(&ret, 256, 256, FilterType::Gaussian);
    Ok(x256)
}

pub fn make_icon(path: impl AsRef<Path>) -> anyhow::Result<()> {
    let png = sqr_png(&path)?;

    let new_path = path.as_ref().with_file_name(ICON_FILE_NAME);

    png.save_with_format(&new_path, ImageFormat::Ico)?;
    set_attr_hidden(new_path)
}
