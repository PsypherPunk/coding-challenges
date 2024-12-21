use ccwc::{number_of_bytes, number_of_characters, number_of_lines, number_of_words};

#[test]
fn count_bytes() {
    let bytes_read = number_of_bytes("tests/test.txt");

    assert_eq!(Ok(342_190), bytes_read);
}

#[test]
fn count_lines() {
    let lines_read = number_of_lines("tests/test.txt");

    assert_eq!(Ok(7_145), lines_read);
}

#[test]
fn count_words() {
    let words_read = number_of_words("tests/test.txt");

    assert_eq!(Ok(58_164), words_read);
}

#[test]
fn count_chars() {
    let chars_read = number_of_characters("tests/test.txt");

    assert_eq!(Ok(339_292), chars_read);
}
