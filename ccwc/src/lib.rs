use std::io::{BufRead, BufReader, Error as IoError, Read};

#[derive(Debug)]
pub enum CcwcError {
    IoError(IoError),
}

impl From<IoError> for CcwcError {
    fn from(error: IoError) -> Self {
        Self::IoError(error)
    }
}

#[derive(Default)]
pub struct CcwcCount {
    pub characters: usize,
    pub words: usize,
    pub lines: usize,
    pub bytes: usize,
}

impl<R: Read> From<R> for CcwcCount {
    fn from(input: R) -> Self {
        let reader = BufReader::new(input);

        reader
            .lines()
            .try_fold(Self::default(), |mut acc, line| match line {
                Ok(line) => {
                    acc.lines += 1;
                    acc.characters += line.chars().count();
                    acc.words += line.split_whitespace().count();
                    acc.bytes += line.as_bytes().len();
                    Ok(acc)
                }
                Err(_) => Err(acc),
            })
            .unwrap_or_default()
    }
}
