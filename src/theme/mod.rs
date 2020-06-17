use crate::color::Color;

mod dracula;

pub trait Theme {
    fn default() -> Color;

    fn keyword() -> Color;

    fn comment() -> Color;

    fn background() -> Color;
}
