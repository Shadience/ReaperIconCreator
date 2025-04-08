/*
TO DO:
- Result will be named same as input image
- Auto move to REAPER/Data/toolbar-icons
- Create application interface
- Make web version
*/

use image::{imageops::{resize, FilterType::{self}}, open, DynamicImage, GenericImage, RgbaImage};

const ON_HOVER_BRIGHTNESS: i32 = -25;
const CLICKED_CONTRAST: f32 = 10f32;
const FILTER_TYPE: FilterType = FilterType::CatmullRom;

fn main() {
    image_to_icon(&open("example/original.jpg").unwrap());
    image_to_icon150(&open("example/original.jpg").unwrap());
    image_to_icon200(&open("example/original.jpg").unwrap());
}

fn image_to_icon(img: &DynamicImage) {
    let img = DynamicImage::ImageRgba8(resize(img, 30, 30, FILTER_TYPE)); // open change to input function
    let mut res = RgbaImage::new(90, 30);

    res.copy_from(&img, 0, 0).ok(); // normal
    res.copy_from(&img.brighten(ON_HOVER_BRIGHTNESS), 30, 0).ok(); // on mouse hover
    res.copy_from(&img.adjust_contrast(CLICKED_CONTRAST), 60, 0).ok(); // clicked

    res.save_with_format("example/result.png", image::ImageFormat::Png).ok();
}
fn image_to_icon150(img: &DynamicImage) {
    let img = DynamicImage::ImageRgba8(resize(img, 45, 45, FILTER_TYPE)); // open change to input function
    let mut res = RgbaImage::new(135, 45);

    res.copy_from(&img, 0, 0).ok(); // normal
    res.copy_from(&img.brighten(ON_HOVER_BRIGHTNESS), 45, 0).ok(); // on mouse hover
    res.copy_from(&img.adjust_contrast(CLICKED_CONTRAST), 90, 0).ok(); // clicked

    res.save_with_format("example/result150.png", image::ImageFormat::Png).ok();
}
fn image_to_icon200(img: &DynamicImage) {
    let img = DynamicImage::ImageRgba8(resize(img, 60, 60, FILTER_TYPE)); // open change to input function
    let mut res = RgbaImage::new(180, 60);

    res.copy_from(&img, 0, 0).ok(); // normal
    res.copy_from(&img.brighten(ON_HOVER_BRIGHTNESS), 60, 0).ok(); // on mouse hover
    res.copy_from(&img.adjust_contrast(CLICKED_CONTRAST), 120, 0).ok(); // clicked

    res.save_with_format("example/result200.png", image::ImageFormat::Png).ok();
}