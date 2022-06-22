pub mod hasher;
pub mod operations;
pub mod reader;

#[cfg(test)]
mod test_utils {
    use rand::{self, distributions::Alphanumeric, Rng};
    use std::fs;

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
                Err(e) => panic!(
                    "Failed to open temp file! Error: {}, FileMode: {:?}, TempFileHandler: {:?}",
                    e, file_mode, self
                ),
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
}
