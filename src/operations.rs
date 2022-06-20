use crate::cli::{Commands, ProgramOptions};
use crate::{cli, hasher, reader};
use std::io::{stdin, BufRead, Write};
use std::{fs, path};

const UNKNOWN_ERROR_MSG: &str = "An unknown error occurred";
const UNKNOWN_ERROR_EXIT_CODE: i32 = 10;
const INPUT_READ_ERROR: i32 = 4;

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
        }
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

pub struct RainbowTableGenerator {
    pub word_file_path: String,
    pub rainbow_table_file_path: String,
}

impl RainbowTableGenerator {
    pub fn new(word_file_path: &str, rainbow_table_file_path: &str) -> RainbowTableGenerator {
        RainbowTableGenerator {
            word_file_path: String::from(word_file_path),
            rainbow_table_file_path: String::from(rainbow_table_file_path),
        }
    }

    fn write_hashes_to_file<R: BufRead>(
        self,
        mut reader: R,
        rainbow_table_file_path: &str,
        serialized_hashes: Vec<String>,
    ) -> i32 {
        // Check if file exists, and if it does, prompt to overwrite
        let path_exists = path::Path::new(rainbow_table_file_path).exists();
        if path_exists {
            eprintln!(
                "{} already exists. Overwrite? (Y/n)",
                rainbow_table_file_path
            );
            let mut buf = String::new();
            if reader.read_line(&mut buf).is_err() {
                eprintln!("Error while reading input!");
                return INPUT_READ_ERROR;
            }
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
                reader::FILE_OPERATION_ERROR
            }
            Ok(_) => 0,
        }
    }

    pub fn run(self, generate_table_options: cli::GenerateTableOptions) -> i32 {
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
        self.write_hashes_to_file(stdin.lock(), rainbow_table_file_path, serialized_hashes);
        println!("Write complete!");
        0
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hasher::{serialize_hashes, HASH_DELIMITER};
    use crate::test_utils;
    use std::io::{BufReader, BufWriter, Read};

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
            &input[..],
            &temp_file_handler.temp_file_path,
            serialized_hashes,
        );

        // Verify that the expected things were written to the file
        let wordfile = temp_file_handler.get_file_object(test_utils::FileMode::Read);
        let reader = BufReader::new(wordfile);
        let expected_lines = vec![
            format!(
                "potato{}e91c254ad58860a02c788dfb5c1a65d6a8846ab1dc649631c7db16fef4af2dec",
                HASH_DELIMITER
            ),
            format!(
                "rice{}209f76418ece7c936b65ff4777a578d860f762c37ad6c7f08f5826242199ef51",
                HASH_DELIMITER
            ),
            format!(
                "noodles{}838f8d9acc45bd36e3213c47c3222e644f44c959fa370bbfa6df46b171c02f0c",
                HASH_DELIMITER
            ),
            format!(
                "salad{}c6c3fa689e291bba6f7436ee76dc542ec4678a410a2adbb26bbedfd1e6a8aa85",
                HASH_DELIMITER
            ),
        ];
        for line in reader.lines() {
            match line {
                Ok(line) => assert!(
                    expected_lines.contains(&line),
                    "Expected line not found in written wordfile: {}",
                    line
                ),
                Err(e) => panic!("{}", e),
            };
        }
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
            Err(e) => panic!("{}", e),
        };
        // Drop file pointer and writer to "close" the file, prevent any conflicts later on
        std::mem::drop(writer);
        std::mem::drop(file);

        let words = vec!["potato".to_string()];
        let serialized_hashes = serialize_hashes(words);
        let input = b"n\n";
        write_hashes_to_file(
            &input[..],
            &temp_file_handler.temp_file_path,
            serialized_hashes,
        );
        // File should not be overwritten
        let file = temp_file_handler.get_file_object(test_utils::FileMode::Read);
        let mut reader = BufReader::new(&file);
        let mut read_text = String::new();
        // Read all text to a string buffer, to make sure that nothing else was appended to the file
        match reader.read_to_string(&mut read_text) {
            // If Ok, read until EOF was successful
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        };
        assert_eq!(sample_text, read_text);
        println!("Test complete!")
    }
}

pub mod generate_table {
    use crate::{cli, hasher, reader};
    use std::io::{stdin, BufRead, Write};
    use std::{fs, path};

    const INPUT_READ_ERROR: i32 = 4;

    fn write_hashes_to_file<R: BufRead>(
        mut reader: R,
        rainbow_table_file_path: &str,
        serialized_hashes: Vec<String>,
    ) -> i32 {
        // Check if file exists, and if it does, prompt to overwrite
        let path_exists = path::Path::new(rainbow_table_file_path).exists();
        if path_exists {
            eprintln!(
                "{} already exists. Overwrite? (Y/n)",
                rainbow_table_file_path
            );
            let mut buf = String::new();
            if reader.read_line(&mut buf).is_err() {
                eprintln!("Error while reading input!");
                return INPUT_READ_ERROR;
            }
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
                reader::FILE_OPERATION_ERROR
            }
            Ok(_) => 0,
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
        use crate::hasher::{serialize_hashes, HASH_DELIMITER};
        use crate::test_utils;
        use std::io::{BufReader, BufWriter, Read};

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
                &input[..],
                &temp_file_handler.temp_file_path,
                serialized_hashes,
            );

            // Verify that the expected things were written to the file
            let wordfile = temp_file_handler.get_file_object(test_utils::FileMode::Read);
            let reader = BufReader::new(wordfile);
            let expected_lines = vec![
                format!(
                    "potato{}e91c254ad58860a02c788dfb5c1a65d6a8846ab1dc649631c7db16fef4af2dec",
                    HASH_DELIMITER
                ),
                format!(
                    "rice{}209f76418ece7c936b65ff4777a578d860f762c37ad6c7f08f5826242199ef51",
                    HASH_DELIMITER
                ),
                format!(
                    "noodles{}838f8d9acc45bd36e3213c47c3222e644f44c959fa370bbfa6df46b171c02f0c",
                    HASH_DELIMITER
                ),
                format!(
                    "salad{}c6c3fa689e291bba6f7436ee76dc542ec4678a410a2adbb26bbedfd1e6a8aa85",
                    HASH_DELIMITER
                ),
            ];
            for line in reader.lines() {
                match line {
                    Ok(line) => assert!(
                        expected_lines.contains(&line),
                        "Expected line not found in written wordfile: {}",
                        line
                    ),
                    Err(e) => panic!("{}", e),
                };
            }
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
                Err(e) => panic!("{}", e),
            };
            // Drop file pointer and writer to "close" the file, prevent any conflicts later on
            std::mem::drop(writer);
            std::mem::drop(file);

            let words = vec!["potato".to_string()];
            let serialized_hashes = serialize_hashes(words);
            let input = b"n\n";
            write_hashes_to_file(
                &input[..],
                &temp_file_handler.temp_file_path,
                serialized_hashes,
            );
            // File should not be overwritten
            let file = temp_file_handler.get_file_object(test_utils::FileMode::Read);
            let mut reader = BufReader::new(&file);
            let mut read_text = String::new();
            // Read all text to a string buffer, to make sure that nothing else was appended to the file
            match reader.read_to_string(&mut read_text) {
                // If Ok, read until EOF was successful
                Ok(_) => (),
                Err(e) => panic!("{}", e),
            };
            assert_eq!(sample_text, read_text);
            println!("Test complete!")
        }
    }
}

pub mod crack_hash {
    use crate::{cli, hasher, reader};

    const CRACK_HASH_RUNTIME_ERROR_EXIT_CODE: i32 = 3;

    fn crack_hash<'a>(
        hash: &String,
        rainbow_table: &'a Vec<hasher::WordHash>,
    ) -> Result<&'a String, ()> {
        for wordhash in rainbow_table {
            if &wordhash.hash == hash {
                return Ok(&wordhash.word);
            }
        }
        Err(())
    }

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
        match crack_hash(&crack_hash_options.hash, &rainbow_table) {
            Ok(cracked_word) => println!("Hash Cracked! The word is: {}", cracked_word),
            Err(_) => println!("Sorry, hash not found in the rainbow table!"),
        };
        0
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::hasher::WordHash;
        use crate::test_utils;
        use cli::CrackHashOptions;
        use hasher::HASH_DELIMITER;
        use std::io::{BufWriter, Write};

        #[test]
        fn test_crack_hash() {
            let expected_hash =
                "c10c7396898976bb8c95966eef6b45c81f66be86cdea5c593ae5cba1026cbbb5".to_string();
            let expected_word = "malenia".to_string();
            let rainbow_table = vec![
                WordHash {
                    hash: "1b8c5c045da33a8545e741e5095d8b96296d84ce1ea18a5918518e2a9c8eca98"
                        .to_string(),
                    word: "uchigatana".to_string(),
                },
                WordHash {
                    hash: expected_hash.clone(),
                    word: expected_word.clone(),
                },
            ];

            // Test that expected cracking happens
            match crack_hash(&expected_hash, &rainbow_table) {
                Ok(word) => assert_eq!(*word, expected_word),
                Err(_) => panic!("Failed to crack expected word {}", expected_word),
            };

            // Test that Err is returned when hash is not present in rainbow table
            let absent_word = String::from("test");
            if let Ok(word) = crack_hash(&absent_word, &rainbow_table) {
                panic!(
                    "Word was cracked even though hash was not in rainbow table. Got: {}",
                    word
                );
            }
        }

        #[test]
        fn test_run() {
            let expected_word = "gitlab";
            let expected_hash = "9d96d9d5b1addd7e7e6119a23b1e5b5f68545312bfecb21d1cdc6af22b8628b8";
            let wordlist_lines = vec![
                format!(
                    "hashicorp{}fe64108583908cdaeacb766f4e1c26977727ece6c44dd901ab1f511c32e22dc0",
                    HASH_DELIMITER
                ),
                format!(
                    "pulumi{}fbe2a04069387628783a3f90b947236e6ff8b1c099e710871356a6381a4e20b2",
                    HASH_DELIMITER
                ),
                format!("{}{}{}", expected_word, HASH_DELIMITER, expected_hash),
            ];

            // Write wordlist to file
            let temp_file_handler = test_utils::TempFileHandler::new();
            let file = temp_file_handler.get_file_object(test_utils::FileMode::Write);
            let mut writer = BufWriter::new(file);
            for line in wordlist_lines {
                if let Err(e) = writer.write(line.as_bytes()) {
                    panic!("{}", e);
                }
            }

            let temp_file_path = temp_file_handler.temp_file_path.clone();
            let options = CrackHashOptions {
                hash: expected_hash.to_string(),
                rainbow_table_file_path: temp_file_path,
            };

            let return_code = run(options);
            assert_eq!(return_code, 0);

            // Test that return code 0 is returned even when hash is uncracked
            // Word is "absent"
            let absent_hash =
                "5ad38304b535c2987dbd24657c1a11b884984ff600d9f389deb0d4e634fee792".to_string();
            let temp_file_path = temp_file_handler.temp_file_path.clone();
            let options = CrackHashOptions {
                hash: absent_hash,
                rainbow_table_file_path: temp_file_path,
            };
            let return_code = run(options);
            assert_eq!(return_code, 0);
        }
    }
}
