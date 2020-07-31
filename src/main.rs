pub mod constant;
pub mod error;
pub mod options;
pub mod pprint;

use std::fs::File;

use structopt::StructOpt;

use crate::error::LouisError;
use crate::options::Options;
use crate::pprint::pretty_print;

type Result<T> = std::result::Result<T, LouisError>;

fn main() {
    let opt: Options = Options::from_args();
    let Options { entry, base } = opt;

    let f = File::open(entry).unwrap();
    if let Err(e) = pretty_print(f, base) {
        eprintln!("{}", e);
    }
}
