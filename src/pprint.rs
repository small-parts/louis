use std::fs::File;
use std::io::{BufReader, Read};
use std::iter::repeat;

use crate::constant::{ADDRESS_LABEL, ASCII_LABEL, COLUMNS, HEX_LABEL, HORIZONTAL_DIVIDER, VERTICAL_DIVIDER};
use crate::options::Base;
use crate::Result;

pub fn pretty_print(f: File, base: Base) -> Result<()> {
    let metadata = f.metadata()?;
    let file_size = metadata.len();
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::<u8>::with_capacity(metadata.len() as usize);
    reader.read_to_end(&mut buffer)?; // TODO: use stream when read a big file

    let address_label_length = ADDRESS_LABEL
        .len()
        .max(f64::log(file_size as f64, base.into()).ceil() as usize);

    pretty_print_header(address_label_length);
    pretty_print_body(address_label_length, &buffer);
    pretty_print_footer(address_label_length);

    Ok(())
}

fn pretty_print_header(address_label_length: usize) {
    let horizontal_dividers = repeat(HORIZONTAL_DIVIDER);

    println!(
        "┌{address_border}┬{data_border}┬{ascii_border}┐",
        address_border = horizontal_dividers
            .clone()
            .take(address_label_length)
            .collect::<String>(),
        data_border = horizontal_dividers.clone().take(COLUMNS * 3 - 1).collect::<String>(),
        ascii_border = horizontal_dividers.take(COLUMNS).collect::<String>(),
    );
    println!(
        "{divider}{address:<address_label_length$}{divider}{data:^data_label_length$}{divider}{ascii:^ascii_label_length$}{divider}",
        divider = VERTICAL_DIVIDER,
        address = ADDRESS_LABEL,
        address_label_length = address_label_length,
        data = HEX_LABEL,
        data_label_length = COLUMNS * 3 - 1,
        ascii  = ASCII_LABEL,
        ascii_label_length = COLUMNS,
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
            print!("{}", repeat(' ').take((COLUMNS - chunk.len()) * 3).collect::<String>());
        }
        print!("{}", VERTICAL_DIVIDER);

        for (no, number) in chunk.iter().enumerate() {
            print!("{}", itoa(*number));
            if no + 1 == chunk.len() {
                print!("{}", repeat(' ').take(COLUMNS - chunk.len()).collect::<String>());
            }
        }

        println!("{}", VERTICAL_DIVIDER);
    }
}

fn pretty_print_footer(address_label_length: usize) {
    let horizontal_dividers = repeat(HORIZONTAL_DIVIDER);

    println!(
        "└{}┴{}┴{}┘",
        horizontal_dividers
            .clone()
            .take(address_label_length)
            .collect::<String>(),
        horizontal_dividers.clone().take(COLUMNS * 3 - 1).collect::<String>(),
        horizontal_dividers.take(COLUMNS).collect::<String>()
    );
}

fn itoa(i: u8) -> char {
    if i.is_ascii_alphanumeric() {
        i as char
    } else {
        '.'
    }
}
