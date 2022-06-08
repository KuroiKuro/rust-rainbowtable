use std::fs;
use std::io::{BufRead, BufReader, Error, ErrorKind};

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
        Ok(lines) => lines,
    };
    Ok(words)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils;
    use std::fs::{set_permissions, Permissions};
    use std::io::{BufWriter, Write};
    use std::os::unix::fs::PermissionsExt;

    #[test]
    fn test_read_words() {
        let temp_file_handler = test_utils::TempFileHandler::new();
        let temp_file = temp_file_handler.get_file_object(test_utils::FileMode::Write);
        let mut writer = BufWriter::new(&temp_file);
        let words: [&str; 3] = ["isshin", "glock", "saint"];
        let buf = words.join("\n");
        match writer.write_all(buf.as_bytes()) {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        };
        // drop file handle
        std::mem::drop(writer);
        std::mem::drop(temp_file);

        let read_words = match read_words(&temp_file_handler.temp_file_path) {
            Ok(words) => words,
            Err(e) => panic!("{}", e),
        };

        let lines_iter = read_words.into_iter().zip(words.into_iter());
        lines_iter.for_each(|pair| {
            assert_eq!(pair.0, pair.1);
        });
    }

    #[test]
    fn test_read_words_no_file() {
        let nonexistent_file_path = "/abc/defghi/jkl.qwerty";
        match read_words(nonexistent_file_path) {
            Err(e) => assert!(e.contains("File not found")),
            Ok(_) => panic!("Did not fail when it should have failed with 'File not found'"),
        };
    }

    #[test]
    fn test_read_words_no_permissions() {
        let temp_file_handler = test_utils::TempFileHandler::new();
        let permissions = Permissions::from_mode(0o000);
        match set_permissions(&temp_file_handler.temp_file_path, permissions) {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        };

        match read_words(&temp_file_handler.temp_file_path) {
            Err(e) => assert!(e.contains("Permission denied")),
            Ok(_) => panic!("Did not fail when it should have failed with 'Permission Denied'"),
        };
    }
}
