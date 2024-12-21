use std::fs::File;
use std::io::{BufRead, BufReader, Error as IoError, Read};

#[derive(Debug, Eq, PartialEq)]
pub enum CcwcError {
    IoError(String),
}

impl From<IoError> for CcwcError {
    fn from(error: IoError) -> Self {
        CcwcError::IoError(error.to_string())
    }
}

pub fn number_of_characters(path: &str) -> Result<usize, CcwcError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let total_chars = reader.bytes().try_fold(0, |acc, byte| {
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

pub fn number_of_words(path: &str) -> Result<usize, CcwcError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let word_count = reader.lines().try_fold(0, |acc, line| {
        Ok::<usize, IoError>(acc + line?.split_whitespace().count())
    })?;

    Ok(word_count)
}

pub fn number_of_lines(path: &str) -> Result<usize, CcwcError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok(reader.lines().count())
}

pub fn number_of_bytes(path: &str) -> Result<usize, CcwcError> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut buffer = [0; 4096];
    let mut total_bytes_read = 0;

    while let Ok(bytes_read) = reader.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        total_bytes_read += bytes_read;
    }

    Ok(total_bytes_read)
}
