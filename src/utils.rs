use egui::{TextureHandle, Ui};
use image::ImageError;

pub fn load_image_texture(path: &String, ui : &mut Ui) -> Result<TextureHandle, ImageError> {
    let image_result = load_image_from_path(&std::path::Path::new(path.as_str()));
    let image = match image_result {
        Ok(image) => { image.clone() }
        Err(error) => {return Err(error)}
    };

    let texture = ui.ctx().load_texture(path, image);

    Ok(texture)
    //ui.image(&texture, img_size);
}

fn load_image_from_path(path: &std::path::Path) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::io::Reader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}