use std::path::PathBuf;
use std::str::FromStr;

use structopt::StructOpt;

use crate::error::LouisError;

#[derive(StructOpt, Debug)]
#[structopt(name = "A command-line hex viewer", author = "ltoddy - taoliu0509@gmail.com")]
pub struct Options {
    #[structopt(name = "path", parse(from_os_str), help = "The file")]
    pub entry: PathBuf,

    #[structopt(short = "b", long = "base", default_value = "hex", help = "Display data base")]
    pub base: Base,

    #[structopt(long = "non-color")]
    pub non_color: bool,

    #[structopt(long = "skip", default_value = "0", help = "Skip the first N bytes of the file.")]
    pub skip: u64,

    #[structopt(long = "limit", default_value = "18446744073709551615", help = "Only read N bytes from file")]
    pub limit: u64,
}

#[derive(StructOpt, Debug, Copy, Clone)]
pub enum Base {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
}

impl FromStr for Base {
    type Err = LouisError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();

        match s.as_str() {
            "b" | "bin" | "binary" => Ok(Base::Binary),
            "o" | "oct" | "octal" => Ok(Base::Octal),
            "d" | "dec" | "decimal" => Ok(Base::Decimal),
            "h" | "hex" | "hexadecimal" => Ok(Base::Hexadecimal),
            _ => unreachable!(),
        }
    }
}

impl Into<f64> for Base {
    fn into(self) -> f64 {
        match self {
            Base::Binary => 2.0,
            Base::Octal => 8.0,
            Base::Decimal => 10.0,
            Base::Hexadecimal => 16.0,
        }
    }
}

#[allow(clippy::len_without_is_empty)]
impl Base {
    pub fn len(&self) -> usize {
        match self {
            Base::Binary => 8,
            Base::Octal | Base::Decimal => 3,
            Base::Hexadecimal => 2,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Base::Binary => "binary",
            Base::Octal => "octal",
            Base::Decimal => "decimal",
            Base::Hexadecimal => "hexadecimal",
        }
    }
}
