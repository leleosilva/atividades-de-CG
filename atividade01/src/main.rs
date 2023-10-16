use std::{io::{stderr, Write}, path::Path};
use image::RgbImage;

pub struct ImgManager {

}

impl ImgManager {
    pub fn save_png(filename: &str, (width, height): (u32, u32)) {
        let mut image: RgbImage = RgbImage::new(width, height);
        image.save(filename).unwrap();
    }
}

fn main() {
    ImgManager::save_png("output.png", (400, 400));
}