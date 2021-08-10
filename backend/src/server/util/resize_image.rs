use std::path::Path;

use image::{imageops::FilterType, GenericImageView, ImageOutputFormat};

pub fn resize_image(
    path: impl AsRef<Path>,
    width: u32,
) -> anyhow::Result<Vec<u8>> {
    let image = image::io::Reader::open(path)?.decode()?;

    let scale_factor = width as f32 / image.width() as f32;
    let image = image.resize(
        width,
        (image.height() as f32 * scale_factor) as u32,
        FilterType::CatmullRom,
    );

    let mut buf = Vec::new();
    image.write_to(&mut buf, ImageOutputFormat::Jpeg(80))?;

    Ok(buf)
}
