use crate::cli::{ProgramOptions, Commands};
use std::process;


const UNKNOWN_ERROR_MSG: &str = "An unknown error occurred";
const UNKNOWN_ERROR_EXIT_CODE: i32 = 10;


pub fn select_run(mut program_options: ProgramOptions) {
    match program_options.operation {
        Commands::GenerateTable { .. } => {
            let gen_table_opts = program_options.get_generate_table_options();
            match gen_table_opts {
                Some(opts) => generate_table::run(opts),
                None => {
                    // We should not be entering this loop, since clap already parses and confirms
                    // that the required arguments are passed to the program
                    eprintln!("{}", UNKNOWN_ERROR_MSG);
                    process::exit(UNKNOWN_ERROR_EXIT_CODE);
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
                    process::exit(UNKNOWN_ERROR_EXIT_CODE);
                }
            };
        }
    }
}

mod generate_table {
    use std::{fs, process, path};
    use std::io::{stdin, Write, BufRead}; 
    use crate::{cli, reader, hasher};

    fn write_hashes_to_file<R: BufRead>(mut reader: R, rainbow_table_file_path: &str, serialized_hashes: Vec<String>) {
        // Check if file exists, and if it does, prompt to overwrite
        let path_exists = path::Path::new(rainbow_table_file_path).exists();
        if path_exists {
            eprintln!("{} already exists. Overwrite? (Y/n)", rainbow_table_file_path);
            let mut buf = String::new();
            match reader.read_line(&mut buf) {
                Err(_) => {
                    eprintln!("Error while reading input!");
                    process::exit(90);
                },
                _ => ()
            };
            let first_char: char = buf.as_bytes()[0] as char;
            if first_char != 'y' && first_char != 'Y' && first_char != '\n' {
                process::exit(0);
            }
        }

        // Create a new file, and write to it
        let mut file = match fs::File::create(rainbow_table_file_path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Unable to open file for writing: {}", e);
                process::exit(reader::FILE_OPERATION_ERROR.into());
            }
        };

        let mut content = String::new();
        for hash in serialized_hashes {
            content.push_str(&format!("{}\n", &hash));
        }
        match file.write_all(content.as_bytes()) {
            Err(e) => {
                eprintln!("Error while writing hashes to file: {}", e);
                process::exit(reader::FILE_OPERATION_ERROR.into());
            },
            Ok(_) => ()
        }
    }

    pub fn run(generate_table_options: cli::GenerateTableOptions) {
        let word_file_path = &generate_table_options.word_file_path;
        let rainbow_table_file_path = &generate_table_options.rainbow_table_file_path;
        let words = match reader::read_words(word_file_path) {
            Ok(result) => result,
            Err(e) => {
                eprintln!("{}", e);
                process::exit(reader::FILE_OPERATION_ERROR.into());
            }
        };

        println!("Generating words...");
        let serialized_hashes = hasher::serialize_hashes(words);
        println!("Generated {} words", serialized_hashes.len());
        println!("Writing generated words to {}", rainbow_table_file_path);
        let stdin = stdin();
        write_hashes_to_file(stdin.lock(), rainbow_table_file_path, serialized_hashes);
        println!("Write complete!");
    }

    #[cfg(test)]
    mod test_utils {
        use std::fs;
        use std::io::Write;
        use rand::{self, Rng, distributions::Alphanumeric};

        const FILEPATH: &str = "/tmp/test_rainbow_file.txt";
        const TMP_DIR: &str = "/tmp/test_wordlist.txt";
        // The number of characters for the randomly generated temp dir and file names
        const TEMP_NAME_LENGTH: usize = 8;

        pub struct TempFileHandler {
            temp_dir_path: String,
            temp_file_path: String,
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

        }

        impl Drop for TempFileHandler {
            fn drop(&mut self) {
                // Cleanup temp dir and all files within it
                if let Err(e) = fs::remove_dir_all(&self.temp_dir_path) {
                    panic!("{}", e);
                };
            }
        }

        fn setup_wordlist() {
            // Create an example wordlist for testing purposes
            let mut wordfile = match fs::File::create(TMP_DIR) {
                Ok(wordfile) => wordfile,
                Err(e) => panic!("{}",e)
            };
            match writeln!(wordfile, "sample\r\npirate\r\nmagician\r\nhermes\r\n") {
                Ok(_) => (),
                Err(e) => panic!("{}", e)
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::io::LineWriter;

        

        #[test]
        fn test_write_hashes_to_file() {
            
        }
    }
}

mod crack_hash {
    use std::process;
    use crate::{cli, hasher, reader};

    const CRACK_HASH_RUNTIME_ERROR_EXIT_CODE: u8 = 4;

    fn create_rainbow_table(words: Vec<String>) -> Vec<hasher::WordHash> {
        match hasher::deserialize_hashes(words) {
            Ok(hashes) => hashes,
            Err(e) => {
                eprintln!("{}", e);
                process::exit(CRACK_HASH_RUNTIME_ERROR_EXIT_CODE.into());
            }
        }
    }

    pub fn run(crack_hash_options: cli::CrackHashOptions) {
        // Read words from file
        let rainbow_table_file_path = crack_hash_options.rainbow_table_file_path;
        let read_words = match reader::read_words(&rainbow_table_file_path) {
            Ok(result) => result,
            Err(e) => {
                eprintln!("{}", e);
                process::exit(reader::FILE_OPERATION_ERROR.into());
            }
        };
        let rainbow_table = create_rainbow_table(read_words);
        for wordhash in rainbow_table {
            if wordhash.hash == crack_hash_options.hash {
                println!("Hash Cracked! The word is: {}", wordhash.word);
                return;
            }
        }
        println!("Sorry, hash not found in the rainbow table!");
    }
}
