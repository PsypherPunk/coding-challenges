use std::fs::File;
use std::io::{self, Read};

use argh::FromArgs;

use ::ccwc::{number_of_bytes, number_of_characters, number_of_lines, number_of_words, CcwcError};

#[derive(FromArgs)]
#[allow(clippy::struct_excessive_bools)]
#[argh(description = "wc for Coding Challenges")]
struct Ccwc {
    #[argh(
        switch,
        short = 'c',
        description = "outputs the number of bytes in a file"
    )]
    count_bytes: bool,

    #[argh(
        switch,
        short = 'l',
        description = "outputs the number of bytes in a file"
    )]
    count_lines: bool,

    #[argh(
        switch,
        short = 'w',
        description = "outputs the number of words in a file"
    )]
    count_words: bool,

    #[argh(
        switch,
        short = 'm',
        description = "outputs the number of characters in a file"
    )]
    count_chars: bool,

    #[argh(positional)]
    path: Option<String>,
}

enum Input {
    File(File),
    Stdin(io::Stdin),
}

impl Read for Input {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        match self {
            Self::File(file) => file.read(buffer),
            Self::Stdin(stdin) => stdin.read(buffer),
        }
    }
}

fn main() -> Result<(), CcwcError> {
    let ccwc: Ccwc = argh::from_env();

    let mut input = match &ccwc.path {
        Some(path) => Input::File(File::open(path)?),
        None => Input::Stdin(io::stdin()),
    };

    if ccwc.count_bytes {
        match number_of_bytes(&mut input) {
            Ok(b) => {
                println!("{b} {}", ccwc.path.unwrap_or_default());
                Ok(())
            }
            Err(e) => Err(e),
        }
    } else if ccwc.count_lines {
        match number_of_lines(&mut input) {
            Ok(l) => {
                println!("{l} {}", ccwc.path.unwrap_or_default());
                Ok(())
            }
            Err(e) => Err(e),
        }
    } else if ccwc.count_words {
        match number_of_words(&mut input) {
            Ok(w) => {
                println!("{w} {}", ccwc.path.unwrap_or_default());
                Ok(())
            }
            Err(e) => Err(e),
        }
    } else if ccwc.count_chars {
        match number_of_characters(&mut input) {
            Ok(c) => {
                println!("{c} {}", ccwc.path.unwrap_or_default());
                Ok(())
            }
            Err(e) => Err(e),
        }
    } else {
        let bytes = number_of_bytes(&mut input);
        let lines = number_of_lines(&mut input);
        let words = number_of_words(&mut input);
        match (bytes, lines, words) {
            (Ok(b), Ok(l), Ok(w)) => {
                println!("{l} {w} {b} {}", ccwc.path.unwrap_or_default());
            }
            _ => panic!(r#"¯\_(ツ)_/¯"#),
        }
        Ok(())
    }
}
