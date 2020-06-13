mod error;
mod options;

use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use structopt::StructOpt;

use crate::error::Error;
use crate::options::Options;

pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    let opt: Options = Options::from_args();
    let Options { filename } = opt;

    if let Err(e) = read_file(filename) {
        eprintln!("reason: {}", e);
    }
}

pub fn read_file<P: AsRef<Path>>(filename: P) -> Result<()> {
    let filename = filename.as_ref();

    let mut file = File::open(filename)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    for (num, line) in s.lines().enumerate() {
        println!("{}: {}", num, line);
    }

    Ok(())
}
