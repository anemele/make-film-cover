use std::io;
use std::path::Path;
use std::process::Command;

use image::imageops::resize;
use image::imageops::FilterType;
use image::io::Reader as ImReader;
use image::GenericImageView;
use image::ImageBuffer;
use image::ImageFormat;
use image::Rgba;
use image::RgbaImage;

use crate::consts::ICON_FILE_NAME;

type SquaredPng = ImageBuffer<Rgba<u8>, Vec<u8>>;

fn sqr_png(path: impl AsRef<Path>) -> io::Result<SquaredPng> {
    let Ok(img) = ImReader::open(&path)?.decode() else {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "failed to decode image.\nmaybe `{}` has a wrong extension?",
                path.as_ref().display()
            ),
        ));
    };

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

pub fn make_icon(path: impl AsRef<Path>) -> io::Result<bool> {
    let png = sqr_png(&path)?;

    let new_path = path.as_ref().with_file_name(ICON_FILE_NAME);

    let ok1 = png.save_with_format(&new_path, ImageFormat::Ico).is_ok();
    let ok2 = Command::new("attrib.exe")
        .arg("+h")
        .arg(new_path)
        .status()?
        .success();
    Ok(ok1 && ok2)
}
