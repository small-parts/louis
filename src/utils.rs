const NON_PRINTABLE_CHARACTER: char = '•';
const NON_ASCII_CHARACTER: char = '×';

pub fn itoa(i: u8) -> char {
    if !i.is_ascii() {
        NON_ASCII_CHARACTER
    } else if i.is_ascii_punctuation() || i.is_ascii_alphanumeric() {
        i as char
    } else {
        NON_PRINTABLE_CHARACTER
    }
}
