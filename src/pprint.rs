use std::fs::File;
use std::io::{BufReader, Read};
use std::iter;

use crate::constant::{ADDRESS_LABEL, COLUMNS, HEX_LABEL, HORIZONTAL_DIVIDER, VERTICAL_DIVIDER};
use crate::Result;

pub fn pretty_print(f: File) -> Result<()> {
    let metadata = f.metadata()?;
    let file_size = metadata.len();
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::<u8>::with_capacity(metadata.len() as usize);
    reader.read_to_end(&mut buffer)?; // TODO: use stream when read a big file

    let address_label_length = ADDRESS_LABEL
        .len()
        .max(f64::log(file_size as f64, 16.0).ceil() as usize);

    pretty_print_header(address_label_length);
    pretty_print_body(address_label_length, &buffer);
    pretty_print_footer(address_label_length);

    Ok(())
}

fn pretty_print_header(address_label_length: usize) {
    println!(
        "┌{}┬{}┐",
        iter::repeat(HORIZONTAL_DIVIDER)
            .take(address_label_length)
            .collect::<String>(),
        iter::repeat(HORIZONTAL_DIVIDER)
            .take(COLUMNS * 3 - 1)
            .collect::<String>()
    );
    println!(
        "{divider}{address_label:<length$}{divider}{data_label:^23}{divider}",
        divider = VERTICAL_DIVIDER,
        address_label = ADDRESS_LABEL,
        length = address_label_length,
        data_label = HEX_LABEL,
    );
}

fn pretty_print_body(address_label_length: usize, data: &[u8]) {
    let chunks = data.chunks(COLUMNS).collect::<Vec<_>>();
    for (index, chunk) in chunks.iter().enumerate() {
        let address = index * COLUMNS;
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
            print!(
                "{}",
                iter::repeat(' ').take((COLUMNS - chunk.len()) * 3).collect::<String>()
            );
        }
        println!("{}", VERTICAL_DIVIDER);
    }
}

fn pretty_print_footer(address_label_length: usize) {
    println!(
        "└{}┴{}┘",
        iter::repeat(HORIZONTAL_DIVIDER)
            .take(address_label_length)
            .collect::<String>(),
        iter::repeat(HORIZONTAL_DIVIDER).take(23).collect::<String>()
    );
}
