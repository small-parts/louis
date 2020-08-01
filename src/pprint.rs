use std::fs::File;
use std::io::{BufReader, Read};
use std::iter::repeat;
use std::path::Path;

use crate::options::Base;
use crate::utils::itoa;
use crate::Result;

pub struct PrettyPrinter {
    base: Base,
    address_label_length: usize,
    reader: BufReader<File>,
    address_label: String,
    data_label: String,
    ascii_label: String,
}

impl PrettyPrinter {
    const VERTICAL_DIVIDER: char = '│';
    const HORIZONTAL_DIVIDER: char = '─';

    const COLUMNS: usize = 16;

    pub fn new<P: AsRef<Path>>(entry: P, base: Base) -> Result<Self> {
        let f = File::open(entry)?;
        let metadata = f.metadata()?;
        let file_size = metadata.len();
        let address_label = String::from("address");

        let address_label_length = address_label
            .len()
            .max(f64::log(file_size as f64, base.into()).ceil() as usize);
        let reader = BufReader::new(f);

        Ok(Self {
            base,
            address_label_length,
            reader,
            address_label,
            data_label: format!("{} data", base.as_str()),
            ascii_label: String::from("ascii"),
        })
    }

    pub fn pretty_print(&mut self) -> Result<()> {
        let mut buffer = Vec::<u8>::new();
        self.reader.read_to_end(&mut buffer)?; // TODO: use stream when read a big file

        self.pretty_print_header();
        self.pretty_print_body(&buffer);
        self.pretty_print_footer();

        Ok(())
    }

    fn pretty_print_header(&mut self) {
        let horizontal_dividers = repeat(Self::HORIZONTAL_DIVIDER);

        println!(
            "┌{address_border}┬{data_border}┬{ascii_border}┐",
            address_border = horizontal_dividers
                .clone()
                .take(self.address_label_length)
                .collect::<String>(),
            data_border = horizontal_dividers
                .clone()
                .take(Self::COLUMNS * (self.base.len() + 1) - 1)
                .collect::<String>(),
            ascii_border = horizontal_dividers.take(Self::COLUMNS).collect::<String>(),
        );
        println!(
            "{divider}{address:<address_label_length$}{divider}{data:^data_label_length$}{divider}{ascii:^ascii_label_length$}{divider}",
            divider = Self::VERTICAL_DIVIDER,
            address = self.address_label,
            address_label_length = self.address_label_length,
            data = self.data_label,
            data_label_length = Self::COLUMNS * (self.base.len() + 1) - 1,
            ascii = self.ascii_label,
            ascii_label_length = Self::COLUMNS,
        );
    }

    fn pretty_print_body(&mut self, data: &[u8]) {
        let chunks = data.chunks(Self::COLUMNS).collect::<Vec<_>>();
        for (index, chunk) in chunks.iter().enumerate() {
            let address = index * Self::COLUMNS;
            print!(
                "{divider}{address:<0length$x}",
                divider = Self::VERTICAL_DIVIDER,
                address = address,
                length = self.address_label_length
            );
            print!("{}", Self::VERTICAL_DIVIDER);
            for (no, number) in chunk.iter().enumerate() {
                print!(
                    "{}",
                    match self.base {
                        Base::Binary => format!("{:<0length$b}", number, length = self.base.len()),
                        Base::Octal => format!("{:<0length$o}", number, length = self.base.len()),
                        Base::Decimal => format!("{:<0length$}", number, length = self.base.len()),
                        Base::Hexadecimal => format!("{:<0length$x}", number, length = self.base.len()),
                    }
                );
                if no + 1 < chunk.len() {
                    print!(" ")
                }
            }
            if index + 1 == chunks.len() {
                print!(
                    "{}",
                    repeat(' ')
                        .take((Self::COLUMNS - chunk.len()) * (self.base.len() + 1))
                        .collect::<String>()
                );
            }
            print!("{}", Self::VERTICAL_DIVIDER);

            for (no, number) in chunk.iter().enumerate() {
                print!("{}", itoa(*number));
                if no + 1 == chunk.len() {
                    print!("{}", repeat(' ').take(Self::COLUMNS - chunk.len()).collect::<String>());
                }
            }

            println!("{}", Self::VERTICAL_DIVIDER);
        }
    }

    fn pretty_print_footer(&mut self) {
        let horizontal_dividers = repeat(Self::HORIZONTAL_DIVIDER);

        println!(
            "└{}┴{}┴{}┘",
            horizontal_dividers
                .clone()
                .take(self.address_label_length)
                .collect::<String>(),
            horizontal_dividers
                .clone()
                .take(Self::COLUMNS * (self.base.len() + 1) - 1)
                .collect::<String>(),
            horizontal_dividers.take(Self::COLUMNS).collect::<String>()
        );
    }
}
