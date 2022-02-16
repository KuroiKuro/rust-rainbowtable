use crate::cli::{ProgramOptions, AvailableOperations};
use std::process;


pub fn select_run(mut program_options: ProgramOptions) {
    match program_options.operation {
        AvailableOperations::GenerateTable => {
            let gen_table_opts = program_options.get_generate_table_options();
            match gen_table_opts {
                Ok(opts) => generate_table::run(opts),
                Err(e) => {
                    eprintln!("{}", e.0);
                    process::exit(e.1.into());
                }
            }
        },
        AvailableOperations::CrackHash => {
            match program_options.get_crack_hash_options() {
                Ok(opts) => crack_hash::run(opts),
                Err(e) => {
                    eprintln!("{}", e.0);
                    process::exit(e.1.into());
                }
            };
        }
    }
}

pub mod generate_table {
    use std::{fs, process, path};
    use std::io::{stdin, Write}; 
    use crate::{cli, reader, hasher};

    fn write_hashes_to_file(rainbow_table_file_path: &str, serialized_hashes: Vec<String>) {
        // Check if file exists, and if it does, prompt to overwrite
        let path_exists = path::Path::new(rainbow_table_file_path).exists();
        if path_exists {
            eprintln!("{} already exists. Overwrite? (Y/n)", rainbow_table_file_path);
            let mut buf = String::new();
            match stdin().read_line(&mut buf) {
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
        let serialized_hashes = hasher::generate_serialized_hashes(words);
        println!("Generated {} words", serialized_hashes.len());
        println!("Writing generated words to {}", rainbow_table_file_path);
        write_hashes_to_file(rainbow_table_file_path, serialized_hashes);
        println!("Write complete!");
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
