use std::cmp;
use ansi_term::Color::RGB;
use image::{Pixel, GenericImageView, FilterType};
use std::path::Path;
use terminal_size::{Width, Height, terminal_size};

pub fn convert(filename: &str) {
    let (Width(terminal_width), Height(terminal_height)) = terminal_size().unwrap();
    let img = image::open(&Path::new(filename)).unwrap();
    let (image_width, image_height) = &img.dimensions();
    let resized_image = &img.resize(cmp::min(*image_width, terminal_width as u32), cmp::min(*image_height, terminal_height as u32), FilterType::Lanczos3);
    let width = resized_image.width();
    let image_buf = &resized_image.to_rgba();
    image_buf.enumerate_pixels().for_each(|(x, _, p)| {
        let (r, g, b, _) = &p.channels4();
        if width == x + 1{
            println!("{}", RGB(*r, *g, *b).paint("@"));
        } else {
            print!("{}", RGB(*r, *g, *b).paint("@"));
        }
    });
}