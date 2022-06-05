use std::fs;
use std::io::{ErrorKind, BufReader, BufRead, Error};

pub const FILE_OPERATION_ERROR: i32 = 2;

pub fn read_words(fpath: &str) -> Result<Vec<String>, String> {
    /*
        Read the words from the file at `fpath`. Assumes that the words
        in the file are delimited by newlines
    */
    // Open file and handle errors
    let word_file = match fs::File::open(fpath) {
        Ok(f) => f,
        Err(error) => {
            let mut error_base = String::from("Error opening word file for reading: ");
            match error.kind() {
            ErrorKind::NotFound => error_base.push_str("File not found"),
            ErrorKind::PermissionDenied => error_base.push_str("Permission denied"),
            _ => error_base.push_str("Unknown Error"),
        }
        return Err(error_base);
        }
    };

    let reader = BufReader::new(word_file);
    let words: Vec<String> = match reader.lines().collect::<Result<Vec<String>, Error>>() {
        Err(error) => return Err(format!("Error while reading from file: {}", error)),
        Ok(lines) => lines
    };
    Ok(words)
}
