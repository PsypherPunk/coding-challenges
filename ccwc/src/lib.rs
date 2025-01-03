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

#[derive(Default)]
pub struct CcwcCount {
    pub characters: usize,
    pub words: usize,
    pub lines: usize,
    pub bytes: usize,
}

impl From<&mut dyn Read> for CcwcCount {
    fn from(input: &mut dyn Read) -> Self {
        let mut reader = BufReader::new(input);
        let mut line = Vec::new();
        let mut count = Self::default();

        while let Ok(size) = reader.read_until(b'\n', &mut line) {
            if size == 0 {
                break;
            }

            count.lines += 1;
            count.characters += String::from_utf8_lossy(&line).chars().count();
            count.words += String::from_utf8_lossy(&line).split_whitespace().count();
            count.bytes += line.len();

            line.truncate(0);
        }

        count
    }
}
