use std::cmp;
use ansi_term::Color::RGB;
use image::{Pixel, GenericImageView, FilterType, DynamicImage};
use std::path::Path;
use terminal_size::{Width, Height, terminal_size};

struct CharPixel<'a> {
    charpixel: &'a str,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl<'a> CharPixel<'a> {
    fn new(pixel: &image::Rgba<u8>) -> CharPixel {
        let rgba = pixel.to_rgba();
        CharPixel {
            charpixel: "@@",
            r: rgba[0] as u8,
            g: rgba[1] as u8,
            b: rgba[2] as u8,
            a: rgba[3] as u8,
        }
    }

    fn colored_char(&self) -> ansi_term::ANSIGenericString<str> {
        RGB(self.r, self.g, self.b).paint(self.charpixel)
    }
}

const CHAR_SIZE: f64 = 0.5;

fn calculate_size_ratio(image_width: f64, image_height: f64) -> f64 {
    let (Width(terminal_width), Height(terminal_height)) = terminal_size().unwrap();
    let ratio = (terminal_height as f64) / image_height;
    let scaled_width = image_width * ratio / CHAR_SIZE;
    if scaled_width < (terminal_width as f64) {
        ratio / CHAR_SIZE
    } else {
        (terminal_width as f64) / image_width
    }
}

fn calculate_fit_size(img: &DynamicImage) -> (u32, u32) {
    let (image_width , image_height) = img.dimensions();
    let ratio = calculate_size_ratio(image_width as f64, image_height as f64);
    (((image_width as f64) * ratio) as u32, ((image_height as f64) * ratio * CHAR_SIZE) as u32)
}

pub fn convert(filename: &str) {
    let img = image::open(&Path::new(filename)).unwrap();
    let (fit_width, fit_height) = calculate_fit_size(&img);
    let resized_image = &img.resize(fit_width, fit_height, FilterType::Lanczos3);
    let width = resized_image.width();
    let image_buf = &resized_image.to_rgba();
    image_buf.enumerate_pixels().for_each(|(x, _, p)| {
        let char_pixel = CharPixel::new(&p);
        let colored_char = &char_pixel.colored_char();
        if width == x + 1{
            println!("{}", colored_char);
        } else {
            print!("{}", colored_char);
        }
    });
}