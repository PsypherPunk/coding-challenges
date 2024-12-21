use std::io::{BufRead, BufReader, Error as IoError, Read};

#[derive(Debug, Eq, PartialEq)]
pub enum CcwcError {
    IoError(String),
}

impl From<IoError> for CcwcError {
    fn from(error: IoError) -> Self {
        Self::IoError(error.to_string())
    }
}

/// # Errors
///
/// Will return `Err` if there is a failure reading from `input`.
pub fn number_of_characters(input: &mut impl Read) -> Result<usize, CcwcError> {
    let total_chars = input.bytes().try_fold(0, |acc, byte| {
        byte.map(|byte| {
            if byte.is_ascii() || byte & 0xC0 != 0x80 {
                acc + 1
            } else {
                acc
            }
        })
    })?;

    Ok(total_chars)
}

/// # Errors
///
/// Will return `Err` if there is a failure reading from `input`.
pub fn number_of_words(input: &mut impl Read) -> Result<usize, CcwcError> {
    let reader = BufReader::new(input);

    let word_count = reader.lines().try_fold(0, |acc, line| {
        Ok::<usize, IoError>(acc + line?.split_whitespace().count())
    })?;

    Ok(word_count)
}

/// # Errors
///
/// Will return `Err` if there is a failure reading from `input`.
pub fn number_of_lines(input: &mut impl Read) -> Result<usize, CcwcError> {
    let reader = BufReader::new(input);

    Ok(reader.lines().count())
}

/// # Errors
///
/// Will return `Err` if there is a failure reading from `input`.
pub fn number_of_bytes(input: &mut impl Read) -> Result<usize, CcwcError> {
    let mut buffer = [0; 4096];
    let mut total_bytes_read = 0;

    while let Ok(bytes_read) = input.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        total_bytes_read += bytes_read;
    }

    Ok(total_bytes_read)
}
