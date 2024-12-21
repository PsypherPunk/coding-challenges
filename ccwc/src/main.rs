use ::ccwc::{number_of_bytes, number_of_characters, number_of_lines, number_of_words, CcwcError};

use argh::FromArgs;

#[derive(FromArgs)]
#[argh(description = "blah")]
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
    path: String,
}

fn main() -> Result<(), CcwcError> {
    let ccwc: Ccwc = argh::from_env();

    if ccwc.count_bytes {
        match number_of_bytes(&ccwc.path) {
            Ok(c) => {
                println!("{} {}", c, ccwc.path);
                Ok(())
            }
            Err(e) => Err(e),
        }
    } else if ccwc.count_lines {
        match number_of_lines(&ccwc.path) {
            Ok(c) => {
                println!("{} {}", c, ccwc.path);
                Ok(())
            }
            Err(e) => Err(e),
        }
    } else if ccwc.count_words {
        match number_of_words(&ccwc.path) {
            Ok(c) => {
                println!("{} {}", c, ccwc.path);
                Ok(())
            }
            Err(e) => Err(e),
        }
    } else if ccwc.count_chars {
        match number_of_characters(&ccwc.path) {
            Ok(c) => {
                println!("{} {}", c, ccwc.path);
                Ok(())
            }
            Err(e) => Err(e),
        }
    } else {
        let bytes = number_of_bytes(&ccwc.path);
        let lines = number_of_lines(&ccwc.path);
        let words = number_of_words(&ccwc.path);
        match (bytes, lines, words) {
            (Ok(b), Ok(l), Ok(w)) => println!("{} {} {} {}", l, w, b, ccwc.path),
            _ => panic!(r#"¯\_(ツ)_/¯"#),
        }
        Ok(())
    }
}
