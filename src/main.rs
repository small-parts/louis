use std::f64;
use std::fs::File;
use std::io::{BufReader, Read};
use std::iter;

use structopt::StructOpt;

use crate::options::Options;

pub mod options;

const VERTICAL_DIVIDER: char = '│';
const HORIZONTAL_DIVIDER: char = '─';

fn main() {
    let opt: Options = Options::from_args();

    let Options { entry } = opt;

    let fp = File::open(entry).unwrap();
    let meta = fp.metadata().unwrap();
    let file_size = meta.len();

    let address_label_length = "address".len().max(f64::log(file_size as f64, 16.0).ceil() as usize);
    let mut reader = BufReader::new(fp);
    let mut buffer = Vec::with_capacity(2 << 10);
    reader.read_to_end(&mut buffer).unwrap();
    let chunks = buffer.chunks(8).collect::<Vec<_>>();

    println!(
        "┌{}┬{}┐",
        iter::repeat(HORIZONTAL_DIVIDER)
            .take(address_label_length)
            .collect::<String>(),
        iter::repeat(HORIZONTAL_DIVIDER).take(23).collect::<String>()
    );
    println!(
        "{divider}{address_label:<length$}{divider}{data_label:^23}{divider}",
        divider = VERTICAL_DIVIDER,
        address_label = "address",
        length = address_label_length,
        data_label = "hexadecimal data",
    );

    for (index, chunk) in chunks.iter().enumerate() {
        let address = index * 8;
        print!(
            "{divider}{address:<0length$x}",
            divider = VERTICAL_DIVIDER,
            address = address,
            length = address_label_length
        );
        print!("{}", VERTICAL_DIVIDER);
        for (no, number) in chunk.iter().enumerate() {
            print!("{:<02X}", number);
            if no + 1 < chunk.len() {
                print!(" ")
            }
        }
        if index + 1 == chunks.len() {
            print!("{}", iter::repeat(' ').take((8 - chunk.len()) * 3).collect::<String>());
        }
        println!("{}", VERTICAL_DIVIDER);
    }

    println!(
        "└{}┴{}┘",
        iter::repeat(HORIZONTAL_DIVIDER)
            .take(address_label_length)
            .collect::<String>(),
        iter::repeat(HORIZONTAL_DIVIDER).take(23).collect::<String>()
    );
}
