use image::*;
use webp::*;

pub fn dy_image_to_web_p(data: Vec<u8>) -> Result<Vec<u8>, ()> {
    // Using `image` crate, open the included .jpg file
    let img: DynamicImage = match image::load_from_memory(&data) {
        Ok(img) => img,
        Err(_) => return Err(()),
    };
    
    let (w, h) = img.dimensions();
    // Optionally, resize the existing photo and convert back into DynamicImage
    let size_factor = 1.0;
    let img: DynamicImage = image::DynamicImage::ImageRgba8(imageops::resize(
        &img,
        (w as f64 * size_factor) as u32,
        (h as f64 * size_factor) as u32,
        imageops::FilterType::Triangle,
    ));

    let encoder: Encoder = Encoder::from_image(&img).unwrap();
    let webp: WebPMemory = encoder.encode(90f32);

    let final_buf = &*webp;
    Ok(final_buf.iter().cloned().collect())
}
