use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "A command-line hex viewer", author = "ltoddy - taoliu0509@gmail.com")]
pub struct Options {
    #[structopt(name = "path", parse(from_os_str))]
    pub entry: PathBuf,
}
