mod color;
mod constants;
mod error;
mod margin;
mod options;
mod theme;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use image::{ImageBuffer, Rgb};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use structopt::StructOpt;

use crate::color::Color;
use crate::constants::MARGIN;
use crate::constants::PADDING;
use crate::error::Error;
use crate::options::Options;

pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    let opt: Options = Options::from_args();
    let Options {
        filename,
        color,
        start,
        end,
    } = opt;

    if let Err(e) = read_file(filename, color, start, end) {
        eprintln!("{}", e);
    }
}

pub fn read_file<P: AsRef<Path>>(filename: P, color: Color, start: Option<usize>, end: Option<usize>) -> Result<()> {
    let filename = filename.as_ref();
    let reader = BufReader::new(File::open(filename)?);
    let lines = reader
        .lines()
        .filter_map(std::result::Result::ok)
        .map(|line: String| line.replace("\t", "    "));

    let lines = match (start, end) {
        (Some(start), Some(end)) => lines.skip(start).take(end - start).collect::<Vec<_>>(),
        (Some(start), None) => lines.skip(start).collect::<Vec<_>>(),
        (None, Some(end)) => lines.take(end).collect::<Vec<_>>(),
        (None, None) => lines.collect::<Vec<_>>(),
    };

    let height = ((lines.len() + 1) as u32) * 20 + 2 * (MARGIN + PADDING);
    let width =
        (lines.iter().fold(0, |acc, x| if acc < x.len() { x.len() } else { acc }) as u32) * 10 + 2 * (MARGIN + PADDING);

    let mut imgbuf = ImageBuffer::new(width, height);

    let font = Vec::from(include_bytes!("Monaco.ttf") as &[u8]);
    let font = Font::from_bytes(&font).unwrap();

    let scale = Scale { x: 20.0, y: 20.0 };

    padding_background(&mut imgbuf, color);
    margin::fill_margin(&mut imgbuf, Color::AntiqueWhite);

    for (index, line) in lines.iter().enumerate() {
        draw_text_mut(
            &mut imgbuf,
            Rgb([0u8, 0u8, 255u8]),
            MARGIN + PADDING,
            (index * 20) as u32 + MARGIN + PADDING,
            scale,
            &font,
            &line,
        );
    }
    imgbuf.save("error.png").unwrap();

    Ok(())
}

pub fn padding_background(image_buffer: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, color: Color) {
    for pixel in image_buffer.pixels_mut() {
        *pixel = color.into_rgb();
    }
}
