pub mod generate_table {
    use std::{fs, process};
    use std::io::Write; 
    use crate::{cli, reader, hasher};

    fn write_hashes_to_file(rainbow_table_file_path: &str, hashed_words: Vec<hasher::WordHash>) {
        // Check if file exists, and if it does, prompt to overwrite
        // TODO: Continue after finishing the rest of the function
        // let path_exists = path::Path::new(table_file_path).exists();
        // if path_exists {
        //     eprintln!("{} already exists. Overwrite? (y/n)", table_file_path);
            
        // }
        // Create a new file, and write to it
        let mut file = match fs::File::create(rainbow_table_file_path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Unable to open file for writing: {}", e);
                process::exit(reader::FILE_OPERATION_ERROR.into());
            }
        };

        let mut content = String::new();
        for hash_struct in hashed_words {
            let delim = hasher::HASH_DELIMITER;
            let line = format!("{}{}{}\n", hash_struct.word, delim, hash_struct.hash);
            content.push_str(&line);
        }
        match file.write_all(content.as_bytes()) {
            Err(e) => {
                eprintln!("Error while writing hashes to file: {}", e);
                process::exit(reader::FILE_OPERATION_ERROR.into());
            },
            Ok(_) => ()
        }
    }

    pub fn run(program_options: cli::ProgramOptions) {
        let word_file_path = &program_options.operation_options.word_file_path;
        let rainbow_table_file_path = &program_options.operation_options.rainbow_table_file_path;
        let words = match reader::read_words(word_file_path) {
            Ok(result) => result,
            Err(e) => {
                eprintln!("{}", e);
                process::exit(reader::FILE_OPERATION_ERROR.into());
            }
        };

        // Hash vector of words
        println!("Generating words...");
        let hashed_words = hasher::hash_word_vec(words);
        println!("Generated {} words", hashed_words.len());
        println!("Writing generated words to {}", rainbow_table_file_path);
        write_hashes_to_file(rainbow_table_file_path, hashed_words);
        println!("Write complete!");
    }
}