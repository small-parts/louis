use std::path::PathBuf;
use std::str::FromStr;

use structopt::StructOpt;

use crate::error::LouisError;

#[derive(StructOpt, Debug)]
#[structopt(name = "A command-line hex viewer", author = "ltoddy - taoliu0509@gmail.com")]
pub struct Options {
    #[structopt(name = "path", parse(from_os_str))]
    pub entry: PathBuf,

    #[structopt(short = "b", long = "base", default_value = "hex")]
    pub base: Base,
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
}
