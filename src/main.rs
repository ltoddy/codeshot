mod color;
mod error;
mod options;

use std::fs::File;
use std::io::Read;
use std::path::Path;

use image::{ImageBuffer, Rgb};
use structopt::StructOpt;

use crate::color::Color;
use crate::error::Error;
use crate::options::Options;

pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    let opt: Options = Options::from_args();
    let Options { filename, color } = opt;

    if let Err(e) = read_file(filename, color) {
        eprintln!("reason: {}", e);
    }
}

pub fn read_file<P: AsRef<Path>>(filename: P, color: Color) -> Result<()> {
    let filename = filename.as_ref();

    let mut file = File::open(filename)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;

    let lines = s.lines().collect::<Vec<_>>();
    let height = (lines.len() as u32) * 16;
    let width = (lines.iter().fold(0, |acc, x| if acc < x.len() { x.len() } else { acc }) as u32) * 7;

    println!("image, width: {}, height: {}, color: {}", width, height, color);

    let mut imgbuf = ImageBuffer::new(width, height);
    padding_background(&mut imgbuf, color);
    imgbuf.save("error.png").unwrap();

    Ok(())
}

pub fn padding_background(image_buffer: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, color: Color) {
    for pixel in image_buffer.pixels_mut() {
        *pixel = color.into_rgb();
    }
}
