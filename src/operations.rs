use crate::cli::{ProgramOptions, Commands};

const UNKNOWN_ERROR_MSG: &str = "An unknown error occurred";
const UNKNOWN_ERROR_EXIT_CODE: i32 = 10;


pub fn select_run(mut program_options: ProgramOptions) -> i32 {
    match program_options.operation {
        Commands::GenerateTable { .. } => {
            let gen_table_opts = program_options.get_generate_table_options();
            match gen_table_opts {
                Some(opts) => generate_table::run(opts),
                None => {
                    // We should not be entering this loop, since clap already parses and confirms
                    // that the required arguments are passed to the program
                    eprintln!("{}", UNKNOWN_ERROR_MSG);
                    UNKNOWN_ERROR_EXIT_CODE
                }
            }
        },
        Commands::CrackHash { .. } => {
            let crack_hash_opts = program_options.get_crack_hash_options();
            match crack_hash_opts {
                Some(opts) => crack_hash::run(opts),
                None => {
                    // We should not be entering this loop, since clap already parses and confirms
                    // that the required arguments are passed to the program
                    eprintln!("{}", UNKNOWN_ERROR_MSG);
                    UNKNOWN_ERROR_EXIT_CODE
                }
            }
        }
    }
}

mod generate_table {
    use std::{fs, path};
    use std::io::{stdin, Write, BufRead}; 
    use crate::{cli, reader, hasher};

    const INPUT_READ_ERROR: i32 = 4;

    fn write_hashes_to_file<R: BufRead>(mut reader: R, rainbow_table_file_path: &str, serialized_hashes: Vec<String>) -> i32 {
        // Check if file exists, and if it does, prompt to overwrite
        let path_exists = path::Path::new(rainbow_table_file_path).exists();
        if path_exists {
            eprintln!("{} already exists. Overwrite? (Y/n)", rainbow_table_file_path);
            let mut buf = String::new();
            match reader.read_line(&mut buf) {
                Err(_) => {
                    eprintln!("Error while reading input!");
                    return INPUT_READ_ERROR;
                },
                _ => ()
            };
            let first_char: char = buf.as_bytes()[0] as char;
            if first_char != 'y' && first_char != 'Y' && first_char != '\n' {
                return 0;
            }
        }

        // Create a new file, and write to it
        let mut file = match fs::File::create(rainbow_table_file_path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Unable to open file for writing: {}", e);
                return reader::FILE_OPERATION_ERROR;
            }
        };

        let mut content = String::new();
        for hash in serialized_hashes {
            content.push_str(&format!("{}\n", &hash));
        }
        match file.write_all(content.as_bytes()) {
            Err(e) => {
                eprintln!("Error while writing hashes to file: {}", e);
                return reader::FILE_OPERATION_ERROR;
            },
            Ok(_) => 0
        }
    }

    pub fn run(generate_table_options: cli::GenerateTableOptions) -> i32 {
        let word_file_path = &generate_table_options.word_file_path;
        let rainbow_table_file_path = &generate_table_options.rainbow_table_file_path;
        let words = match reader::read_words(word_file_path) {
            Ok(result) => result,
            Err(e) => {
                eprintln!("{}", e);
                return reader::FILE_OPERATION_ERROR;
            }
        };

        println!("Generating words...");
        let serialized_hashes = hasher::serialize_hashes(words);
        println!("Generated {} words", serialized_hashes.len());
        println!("Writing generated words to {}", rainbow_table_file_path);
        let stdin = stdin();
        write_hashes_to_file(stdin.lock(), rainbow_table_file_path, serialized_hashes);
        println!("Write complete!");
        0
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use super::super::test_utils;
        use std::io::{BufReader, BufWriter, Read};
        use crate::hasher::{serialize_hashes, HASH_DELIMITER};

        #[test]
        fn test_write_hashes_to_file() {
            // Create a temp file for the test to write to
            let temp_file_handler = test_utils::TempFileHandler::new();

            // Create serialized hashes vec for testing
            let words = vec![
                "potato".to_string(),
                "rice".to_string(),
                "noodles".to_string(),
                "salad".to_string(),
            ];
            let serialized_hashes = serialize_hashes(words);

            let input = b"y\n";
            // https://stackoverflow.com/questions/28370126/how-can-i-test-stdin-and-stdout
            write_hashes_to_file(
                &input[..], &temp_file_handler.temp_file_path, serialized_hashes
            );

            // Verify that the expected things were written to the file
            let wordfile = temp_file_handler.get_file_object(test_utils::FileMode::Read);
            let reader = BufReader::new(wordfile);
            let expected_lines = vec![
                format!("potato{}e91c254ad58860a02c788dfb5c1a65d6a8846ab1dc649631c7db16fef4af2dec", HASH_DELIMITER),
                format!("rice{}209f76418ece7c936b65ff4777a578d860f762c37ad6c7f08f5826242199ef51", HASH_DELIMITER),
                format!("noodles{}838f8d9acc45bd36e3213c47c3222e644f44c959fa370bbfa6df46b171c02f0c", HASH_DELIMITER),
                format!("salad{}c6c3fa689e291bba6f7436ee76dc542ec4678a410a2adbb26bbedfd1e6a8aa85", HASH_DELIMITER),
            ];
            for line in reader.lines() {
                match line {
                    Ok(line) => assert!(expected_lines.contains(&line), "Expected line not found in written wordfile: {}", line),
                    Err(e) => panic!("{}", e)
                };
            };
        }

        #[test]
        fn test_write_hashes_to_file_not_overwrite() {
            // Create a temp file with sample text in it to check if it got overwritten
            let sample_text = "The quick brown fox jumps over the lazy dog.";
            let temp_file_handler = test_utils::TempFileHandler::new();
            let file = temp_file_handler.get_file_object(test_utils::FileMode::Write);
            // Write sample text into file
            let mut writer = BufWriter::new(&file);
            match writer.write_all(sample_text.as_bytes()) {
                Ok(_) => (),
                Err(e) => panic!("{}", e)
            };
            // Drop file pointer and writer to "close" the file, prevent any conflicts later on
            std::mem::drop(writer);
            std::mem::drop(file);

            let words = vec![
                "potato".to_string(),
            ];
            let serialized_hashes = serialize_hashes(words);
            let input = b"n\n";
            write_hashes_to_file(
                &input[..], &temp_file_handler.temp_file_path, serialized_hashes
            );
            // File should not be overwritten
            let file = temp_file_handler.get_file_object(test_utils::FileMode::Read);
            let mut reader = BufReader::new(&file);
            let mut read_text = String::new();
            // Read all text to a string buffer, to make sure that nothing else was appended to the file
            match reader.read_to_string(&mut read_text) {
                // If Ok, read until EOF was successful
                Ok(_) => (),
                Err(e) => panic!("{}", e)
            };
            assert_eq!(sample_text, read_text);
            println!("Test complete!")

        }

    }
}

mod crack_hash {
    use crate::{cli, hasher, reader};

    const CRACK_HASH_RUNTIME_ERROR_EXIT_CODE: i32 = 3;

    pub fn run(crack_hash_options: cli::CrackHashOptions) -> i32 {
        // Read words from file
        let rainbow_table_file_path = crack_hash_options.rainbow_table_file_path;
        let read_words = match reader::read_words(&rainbow_table_file_path) {
            Ok(result) => result,
            Err(e) => {
                eprintln!("{}", e);
                return reader::FILE_OPERATION_ERROR;
            }
        };
        let rainbow_table = match hasher::deserialize_hashes(read_words) {
            Ok(hashes) => hashes,
            Err(e) => {
                eprintln!("{}", e);
                return CRACK_HASH_RUNTIME_ERROR_EXIT_CODE;
            }
        };
        for wordhash in rainbow_table {
            if wordhash.hash == crack_hash_options.hash {
                println!("Hash Cracked! The word is: {}", wordhash.word);
                return 0;
            }
        }
        println!("Sorry, hash not found in the rainbow table!");
        0
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_create_rainbow_table() {
            let words = vec![
                "hashicorp",
                "pulumi",
                "gitlab",
            ];
            
        }
    }

}


#[cfg(test)]
mod test_utils {
    use std::fs;
    use std::io::Write;
    use rand::{self, Rng, distributions::Alphanumeric};

    // The number of characters for the randomly generated temp dir and file names
    const TEMP_NAME_LENGTH: usize = 8;

    #[derive(Debug, PartialEq)]
    pub enum FileMode {
        Read,
        Write,
        Append,
    }

    #[derive(Debug)]
    pub struct TempFileHandler {
        temp_dir_path: String,
        pub temp_file_path: String,
        temp_file_name: String,
    }

    impl TempFileHandler {
        pub fn new() -> TempFileHandler {
            let dir_name = format!("/tmp/{}", TempFileHandler::generate_name());
            let file_name = TempFileHandler::generate_name();
            let file_path = format!("{}/{}", dir_name, file_name);

            if let Err(e) = fs::create_dir(&dir_name) {
                panic!("{}", e)
            };
            if let Err(e) = fs::File::create(&file_path) {
                panic!("{}", e)
            };

            TempFileHandler { 
                temp_dir_path: dir_name,
                temp_file_path: file_path,
                temp_file_name: file_name
            }
        }

        fn generate_name() -> String {
            // https://stackoverflow.com/questions/54275459/how-do-i-create-a-random-string-by-sampling-from-alphanumeric-characters
            let name: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(TEMP_NAME_LENGTH)
                .map(char::from)
                .collect();
            name
        }

        pub fn get_file_object(&self, file_mode: FileMode) -> fs::File {
            let f = fs::OpenOptions::new()
                .append(file_mode == FileMode::Append)
                .read(file_mode == FileMode::Read)
                .write(file_mode == FileMode::Write)
                .open(&self.temp_file_path);
            match f {
                Ok(f) => f,
                Err(e) => panic!("Failed to open temp file! Error: {}, FileMode: {:?}, TempFileHandler: {:?}", e, file_mode, self)
            }
        }

    }

    impl Drop for TempFileHandler {
        fn drop(&mut self) {
            // Cleanup temp dir and all files within it
            if let Err(e) = fs::remove_dir_all(&self.temp_dir_path) {
                panic!("{}", e);
            };
        }
    }

    pub fn setup_wordlist(mut wordfile: &fs::File) {
        match writeln!(wordfile, "sample\r\npirate\r\nmagician\r\nhermes\r\n") {
            Ok(_) => (),
            Err(e) => panic!("{}", e)
        }
    }
}