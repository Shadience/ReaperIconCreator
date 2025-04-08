use image::{imageops::{resize, FilterType::Lanczos3}, open, DynamicImage, GenericImage, RgbaImage};

fn main() {
    image_to_icon(&open("example/images.jpg").unwrap());
}

fn image_to_icon(img: &DynamicImage) {
    let img = DynamicImage::ImageRgba8(resize(img, 30, 30, Lanczos3)); // open change to input function
    let mut res = RgbaImage::new(90, 30);
    res.copy_from(&img, 0, 0).ok();
    res.copy_from(&img.brighten(-25), 30, 0).ok();
    res.copy_from(&img.adjust_contrast(10f32), 60, 0).ok();
    res.save_with_format("example/result.png", image::ImageFormat::Png).ok();
}