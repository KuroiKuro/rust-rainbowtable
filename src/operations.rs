pub mod generate_table {
    use std::{fs, process};
    use std::io::Write; 
    use crate::{cli, reader, hasher};

    fn write_hashes_to_file(rainbow_table_file_path: &str, serialized_hashes: Vec<String>) {
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

        println!("Generating words...");
        let serialized_hashes = hasher::generate_serialized_hashes(words);
        println!("Generated {} words", serialized_hashes.len());
        println!("Writing generated words to {}", rainbow_table_file_path);
        write_hashes_to_file(rainbow_table_file_path, serialized_hashes);
        println!("Write complete!");
    }
}