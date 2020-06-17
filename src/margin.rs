use image::{ImageBuffer, Rgb};

use crate::color::Color;
use crate::constants::MARGIN;

pub fn fill_margin(image_buffer: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, color: Color) {
    let (width, height) = image_buffer.dimensions();

    // from left to right
    for x in 0..width {
        for y in 0..MARGIN {
            let pixel = image_buffer.get_pixel_mut(x, y);
            *pixel = color.into_rgb();
        }
    }
    for x in 0..width {
        for y in (height - MARGIN)..height {
            let pixel = image_buffer.get_pixel_mut(x, y);
            *pixel = color.into_rgb();
        }
    }

    // from top to down
    for x in 0..MARGIN {
        for y in 0..height {
            let pixel = image_buffer.get_pixel_mut(x, y);
            *pixel = color.into_rgb();
        }
    }
    for x in (width - MARGIN)..width {
        for y in 0..height {
            let pixel = image_buffer.get_pixel_mut(x, y);
            *pixel = color.into_rgb();
        }
    }
}
