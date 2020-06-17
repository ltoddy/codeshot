use std::path::PathBuf;

use structopt::StructOpt;

use crate::color::Color;

#[derive(StructOpt, Debug)]
#[structopt(name = "code shot!", author = "ltoddy <taoliu0509@gmail.com>")]
pub struct Options {
    #[structopt(name = "path", parse(from_os_str))]
    pub filename: PathBuf,

    #[structopt(name = "color", long = "color", default_value = "AliceBlue")]
    pub color: Color,

    #[structopt(name = "start", long = "start")]
    pub start: Option<usize>,

    #[structopt(name = "end", long = "end")]
    pub end: Option<usize>,
}
