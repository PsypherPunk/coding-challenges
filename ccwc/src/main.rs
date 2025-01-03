use std::fs::File;
use std::io::{self, Read};

use argh::FromArgs;

use ::ccwc::{CcwcCount, CcwcError};

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
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            Self::File(file) => file.read(buf),
            Self::Stdin(stdin) => stdin.read(buf),
        }
    }
}

fn main() -> Result<(), CcwcError> {
    let ccwc: Ccwc = argh::from_env();

    let mut input = match &ccwc.path {
        Some(path) => Input::File(File::open(path)?),
        None => Input::Stdin(io::stdin()),
    };

    let counts = CcwcCount::from(&mut input as &mut dyn Read);

    if ccwc.count_bytes {
        println!("{} {}", counts.bytes, ccwc.path.unwrap_or_default());
    } else if ccwc.count_lines {
        println!("{} {}", counts.lines, ccwc.path.unwrap_or_default());
    } else if ccwc.count_words {
        println!("{} {}", counts.words, ccwc.path.unwrap_or_default());
    } else if ccwc.count_chars {
        println!("{} {}", counts.characters, ccwc.path.unwrap_or_default());
    } else {
        println!(
            "{} {} {} {}",
            counts.lines,
            counts.words,
            counts.bytes,
            ccwc.path.unwrap_or_default()
        );
    }

    Ok(())
}
