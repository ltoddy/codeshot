use std::fmt;
use std::str::FromStr;

use image::Rgb;

use crate::error::Error;

// see more: https://www.w3schools.com/cssref/css_colors.asp
#[derive(Debug, Clone, Copy)]
pub enum Color {
    AliceBlue,
    AntiqueWhite,
    Aqua,
}

impl Color {
    pub fn into_rgb(self) -> Rgb<u8> {
        match self {
            Color::AliceBlue => Rgb([240, 248, 255]),
            Color::AntiqueWhite => Rgb([250, 235, 215]),
            Color::Aqua => Rgb([0, 255, 255]),
        }
    }
}

impl FromStr for Color {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AliceBlue" => Ok(Color::AliceBlue),
            "AntiqueWhite" => Ok(Color::AntiqueWhite),
            "Aqua" => Ok(Color::Aqua),
            _ => unimplemented!(),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::AliceBlue => write!(f, "AliceBlue"),
            Color::AntiqueWhite => write!(f, "AntiqueWhite"),
            Color::Aqua => write!(f, "Aqua"),
        }
    }
}
