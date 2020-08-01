use crate::constant::{NON_ASCII_CHARACTER, NON_PRINTABLE_CHARACTER};

pub fn itoa(i: u8) -> char {
    if !i.is_ascii() {
        NON_ASCII_CHARACTER
    } else if i.is_ascii_punctuation() || i.is_ascii_alphanumeric() {
        i as char
    } else {
        NON_PRINTABLE_CHARACTER
    }
}
