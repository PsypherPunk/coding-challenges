use ccwc::{number_of_bytes, number_of_characters, number_of_lines, number_of_words};
use std::fs::File;

#[test]
fn count_bytes() {
    let mut input = File::open("tests/test.txt").unwrap();

    let bytes_read = number_of_bytes(&mut input);

    assert_eq!(Ok(342_190), bytes_read);
}

#[test]
fn count_lines() {
    let mut input = File::open("tests/test.txt").unwrap();

    let lines_read = number_of_lines(&mut input);

    assert_eq!(Ok(7_145), lines_read);
}

#[test]
fn count_words() {
    let mut input = File::open("tests/test.txt").unwrap();

    let words_read = number_of_words(&mut input);

    assert_eq!(Ok(58_164), words_read);
}

#[test]
fn count_chars() {
    let mut input = File::open("tests/test.txt").unwrap();

    let chars_read = number_of_characters(&mut input);

    assert_eq!(Ok(339_292), chars_read);
}
