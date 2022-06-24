use crate::{hasher, reader};
use std::io::{stdin, BufRead, Write};
use std::sync::mpsc;
use std::thread;
use std::{fs, path};
use itertools::Itertools;


const CRACK_HASH_RUNTIME_ERROR_EXIT_CODE: i32 = 3;
const INPUT_READ_ERROR: i32 = 4;

fn calculate_thread_counts<T>(threads: usize, mut vec_to_split: Vec<T>) -> Vec<Vec<T>> {
    /*
        Return a vector of Vec<T>, where each Vec<T> has been properly sized into mostly
        equal sizes. In the event vec_to_split cannot be equally sized into n threads, then
        starting from the first split, each split will take 1 extra until the remainder is
        exhausted (probably can phrase this better)
    */ 
    // If the number of threads is bigger than the vec to split, then create vectors of
    // size 1
    let vec_len = vec_to_split.len();
    if threads == 1 {
        return vec![vec_to_split];
    } 

    if threads >= vec_len {
        return vec_to_split.into_iter()
            .map(|item: T| vec![item])
            .collect::<Vec<Vec<T>>>();
    }

    let mut chunk_size = vec_len / threads;
    // The number of threads that need to take 1 extra, so that each thread can handle
    // one additional item, with the exception of the last thread
    let remainder = vec_len % threads;
    if remainder != 0 {
        chunk_size += 1;
    }
    // TODO: Find a more elegant way to write this
    let return_vec: Vec<Vec<T>> = vec![];
    for chunk in vec_to_split.chunks_mut(chunk_size) {
        let mut new_vec = vec![];
        for item in chunk {
            new_vec.push(item);
        }
        return_vec.push(new_vec);
    }
    return_vec
}

pub trait Operator {
    fn run(&self) -> i32;
}

pub struct RainbowTableGenerator {
    pub word_file_path: String,
    pub rainbow_table_file_path: String,
    threads: u32,
}

impl RainbowTableGenerator {
    pub fn new(
        word_file_path: String,
        rainbow_table_file_path: String,
        threads: u32,
    ) -> RainbowTableGenerator {
        RainbowTableGenerator {
            word_file_path,
            rainbow_table_file_path,
            threads,
        }
    }

    fn write_hashes_to_file<R: BufRead>(
        &self,
        mut reader: R,
        serialized_hashes: Vec<String>,
    ) -> i32 {
        // Check if file exists, and if it does, prompt to overwrite
        let path_exists = path::Path::new(&self.rainbow_table_file_path).exists();
        if path_exists {
            eprintln!(
                "{} already exists. Overwrite? (Y/n)",
                &self.rainbow_table_file_path
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
        let mut file = match fs::File::create(&self.rainbow_table_file_path) {
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

    // fn get_serialized_hashes(&self, word_vec: Vec<String>) -> Vec<Vec<String>> {
    //     if self.threads == 1 {
    //         return vec![hasher::serialize_hashes(word_vec)];
    //     }
    //     let threads = self.threads as usize;
    //     let each_vec_size: usize = word_vec.len() / threads;
    //     let remainder = word_vec.len() % threads;

    //     let threads = vec![];
    // }
}

impl Operator for RainbowTableGenerator {
    fn run(&self) -> i32 {
        let words = match reader::read_words(&self.word_file_path) {
            Ok(result) => result,
            Err(e) => {
                eprintln!("{}", e);
                return reader::FILE_OPERATION_ERROR;
            }
        };

        println!("Generating words...");
        let serialized_hashes = hasher::serialize_hashes(words);
        println!("Generated {} words", serialized_hashes.len());
        println!(
            "Writing generated words to {}",
            &self.rainbow_table_file_path
        );
        let stdin = stdin();
        self.write_hashes_to_file(stdin.lock(), serialized_hashes);
        println!("Write complete!");
        0
    }
}

pub struct HashCracker {
    rainbow_table_file_path: String,
    hash: String,
}

impl HashCracker {
    pub fn new(rainbow_table_file_path: String, hash: String) -> HashCracker {
        HashCracker {
            rainbow_table_file_path,
            hash,
        }
    }

    fn crack_hash(&self, rainbow_table: Vec<hasher::WordHash>) -> Result<String, ()> {
        for wordhash in rainbow_table {
            if &wordhash.hash == &self.hash {
                return Ok(wordhash.word);
            }
        }
        Err(())
    }
}

impl Operator for HashCracker {
    fn run(&self) -> i32 {
        // Read words from file
        let read_words = match reader::read_words(&self.rainbow_table_file_path) {
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
        match &self.crack_hash(rainbow_table) {
            Ok(cracked_word) => println!("Hash Cracked! The word is: {}", cracked_word),
            Err(_) => println!("Sorry, hash not found in the rainbow table!"),
        };
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_test_vec(n: usize) -> Vec<String> {
        let test_string = "hello";
        (0..n).into_iter()
            .map(|_| String::from(test_string))
            .collect::<Vec<String>>()
    }

    #[test]
    fn test_calculate_thread_counts() {
        let sample_vec = generate_test_vec(5);
        let calculated_vec = calculate_thread_counts(3, sample_vec);
        // Expected first 2 threads to have size 2, last to have size 1
        println!("calculated: {:?}", calculated_vec);
        assert_eq!(calculated_vec[0].len(), 2);
        assert_eq!(calculated_vec[1].len(), 2);
        assert_eq!(calculated_vec[2].len(), 1);
    }
}


#[cfg(test)]
mod rainbow_table_generator_tests {
    use super::*;
    use crate::hasher::{serialize_hashes, HASH_DELIMITER};
    use crate::test_utils;
    use std::io::{BufReader, BufWriter, Read};

    #[test]
    fn test_rainbow_table_write_hashes_to_file() {
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
        let temp_file_path = String::from(&temp_file_handler.temp_file_path);
        let operator = RainbowTableGenerator::new("".to_string(), temp_file_path, 1);
        let input = b"y\n";
        // https://stackoverflow.com/questions/28370126/how-can-i-test-stdin-and-stdout
        operator.write_hashes_to_file(&input[..], serialized_hashes);

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
        let temp_file_path = String::from(&temp_file_handler.temp_file_path);
        let operator = RainbowTableGenerator::new("".to_string(), temp_file_path, 1);
        let input = b"n\n";
        operator.write_hashes_to_file(&input[..], serialized_hashes);
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

#[cfg(test)]
mod hash_cracker_tests {
    use super::*;
    use crate::hasher::WordHash;
    use crate::test_utils;
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
        let cracker = HashCracker::new("".to_string(), expected_hash);
        match cracker.crack_hash(rainbow_table.to_vec()) {
            Ok(word) => assert_eq!(*word, expected_word),
            Err(_) => panic!("Failed to crack expected word {}", expected_word),
        };

        // Test that Err is returned when hash is not present in rainbow table
        let absent_word_hash =
            String::from("9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08");
        let cracker = HashCracker::new("".to_string(), absent_word_hash);
        if let Ok(word) = cracker.crack_hash(rainbow_table) {
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
        let cracker = HashCracker::new(temp_file_path, expected_hash.to_string());

        let return_code = cracker.run();
        assert_eq!(return_code, 0);

        // Test that return code 0 is returned even when hash is uncracked
        // Word is "absent"
        let absent_hash =
            "5ad38304b535c2987dbd24657c1a11b884984ff600d9f389deb0d4e634fee792".to_string();
        let temp_file_path = temp_file_handler.temp_file_path.clone();
        let cracker = HashCracker::new(temp_file_path, absent_hash);
        let return_code = cracker.run();
        assert_eq!(return_code, 0);
    }
}
