use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "code shot!", author = "ltoddy <taoliu0509@gmail.com>")]
pub(crate) struct Options {
    #[structopt(name = "path", parse(from_os_str))]
    pub(crate) filename: PathBuf,
}
