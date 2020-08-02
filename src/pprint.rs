use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::iter::repeat;

use crate::options::{Base, Options};
use crate::utils::itoa;
use crate::Result;

struct Labels {
    address: (String, usize),
    data: (String, usize),
    ascii: (String, usize),
}

impl Labels {
    fn new(address: (String, usize), data: (String, usize), ascii: (String, usize)) -> Self {
        Self { address, data, ascii }
    }
}

pub struct PrettyPrinter {
    labels: Labels,

    file_size: u64,
    reader: BufReader<File>,

    options: Options,
}

impl PrettyPrinter {
    const VERTICAL_DIVIDER: char = '│';
    const HORIZONTAL_DIVIDER: char = '─';

    const COLUMNS: usize = 16;

    pub fn new(options: Options) -> Result<Self> {
        let f = File::open(&options.entry)?;
        let metadata = f.metadata()?;
        let file_size = metadata.len();

        let address_label = String::from("address");
        let address_label_length = address_label
            .len()
            .max(f64::log(file_size as f64, options.base.into()).ceil() as usize);
        let data_label = format!("{} data", options.base.as_str());
        let data_label_length = Self::COLUMNS * (options.base.len() + 1) - 1;
        let ascii_label = String::from("ascii");
        let ascii_label_length = Self::COLUMNS;

        let labels = Labels::new(
            (address_label, address_label_length),
            (data_label, data_label_length),
            (ascii_label, ascii_label_length),
        );

        let reader = BufReader::new(f);

        Ok(Self {
            labels,
            reader,
            file_size,
            options,
        })
    }

    pub fn pretty_print(&mut self) -> Result<()> {
        let mut buffer = vec![0; self.options.limit.min(self.file_size) as usize];
        self.reader.seek(SeekFrom::Start(self.options.skip))?;
        self.reader.read_exact(&mut buffer)?; // TODO: use stream when read a big file

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
                .take(self.labels.address.1)
                .collect::<String>(),
            data_border = horizontal_dividers
                .clone()
                .take(self.labels.data.1)
                .collect::<String>(),
            ascii_border = horizontal_dividers.take(self.labels.ascii.1).collect::<String>(),
        );
        println!(
            "{divider}{address:<address_label_length$}{divider}{data:^data_label_length$}{divider}{ascii:^ascii_label_length$}{divider}",
            divider = Self::VERTICAL_DIVIDER,
            address = self.labels.address.0,
            address_label_length = self.labels.address.1,
            data = self.labels.data.0,
            data_label_length = self.labels.data.1,
            ascii = self.labels.ascii.0,
            ascii_label_length = self.labels.ascii.1,
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
                length = self.labels.address.1,
            );
            print!("{}", Self::VERTICAL_DIVIDER);
            for (no, number) in chunk.iter().enumerate() {
                print!(
                    "{}",
                    match self.options.base {
                        Base::Binary => format!("{:<0length$b}", number, length = self.options.base.len()),
                        Base::Octal => format!("{:<0length$o}", number, length = self.options.base.len()),
                        Base::Decimal => format!("{:<0length$}", number, length = self.options.base.len()),
                        Base::Hexadecimal => format!("{:<0length$x}", number, length = self.options.base.len()),
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
                        .take((Self::COLUMNS - chunk.len()) * (self.options.base.len() + 1))
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
                .take(self.labels.address.1)
                .collect::<String>(),
            horizontal_dividers
                .clone()
                .take(self.labels.data.1)
                .collect::<String>(),
            horizontal_dividers.take(self.labels.ascii.1).collect::<String>()
        );
    }
}
