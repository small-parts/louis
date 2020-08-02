pub mod color;
pub mod error;
pub mod options;
pub mod pprint;
pub mod utils;

use structopt::StructOpt;

use crate::error::LouisError;
use crate::options::Options;
use crate::pprint::PrettyPrinter;

type Result<T> = std::result::Result<T, LouisError>;

fn main() {
    let opt: Options = Options::from_args();

    if let Err(e) = PrettyPrinter::new(opt).and_then(|mut printer| printer.pretty_print()) {
        eprintln!("{}", e);
    }
}
