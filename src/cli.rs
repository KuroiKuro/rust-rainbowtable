/*
    This module handles the parsing of CLI options.
*/
use std::env;
use std::fmt;
use std::process;

const GENERATE_TABLE_OPERATION: &str = "generate_table";
const CRACK_HASH_OPEATION: &str = "crack_hash";
const CRACK_HASH_OPERATION: &str = "crack_hash";
const MISSING_OPERATION_ARG: &str = "Missing operation argument";
const MISSING_WORD_FILE_ARG: &str = "Missing argument word_file for generate_table operation!";
const MISSING_RAINBOW_TABLE_FILE_ARG: &str = "Missing argument rainbow_table_file for generate_table operation!";
const MISSING_HASH_ARG: &str = "Missing hash argument";

const OPERATION_PARSE_ERROR_EXIT_CODE: u8 = 1;
const ARGUMENT_PARSE_ERROR_EXIT_CODE: u8 = 2;
const GENERATE_TABLE_PARSE_ERROR: u8 = 2;
const CRACK_HASH_PARSE_ERROR: u8 = 3;

/*
    CLI ORDER:
    Generate Table: <program> generate_table <rainbow_table_file_path> <word_file_path>
    Crack Hash: <program> crack_hash <rainbow_table_file_path> <hash>
*/

pub enum AvailableOperations {
    GenerateTable,
    CrackHash,
}

impl fmt::Display for AvailableOperations {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let operation_type = match &self {
            AvailableOperations::GenerateTable => GENERATE_TABLE_OPERATION,
        };
        writeln!(f, "{}", operation_type)
    }
}


pub struct GenerateTableOptions {
    pub word_file_path: String,
    pub rainbow_table_file_path: String,
}

impl GenerateTableOptions {
    pub fn new(mut args: Vec<String>) -> Result<GenerateTableOptions, (String, u8)> {
        let word_file_path = match args.pop() {
            Some(wf) => wf,
            None => return Err((String::from(MISSING_WORD_FILE_ARG), GENERATE_TABLE_PARSE_ERROR))
        };

        let rainbow_table_file_path = match args.pop() {
            Some(wf) => wf,
            None => return Err((String::from(MISSING_RAINBOW_TABLE_FILE_ARG), GENERATE_TABLE_PARSE_ERROR))
        };
        Ok(GenerateTableOptions {
            word_file_path: word_file_path,
            
            rainbow_table_file_path: rainbow_table_file_path
        })
    }
}


impl fmt::Display for GenerateTableOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "word_file: {}, rainbow_table_file: {}", &self.word_file_path, &self.rainbow_table_file_path)
    }
}


pub struct CrackHashOptions {
    pub hash: String,
    pub rainbow_table_file_path: String,
}

impl CrackHashOptions {
    fn new(mut args: Vec<String>) -> Result<CrackHashOptions, (String, u8)> {
        let hash = match args.pop() {
            Some(hash) => hash,
            None => return Err((String::from(MISSING_HASH_ARG), CRACK_HASH_PARSE_ERROR)),
        };

        let rainbow_table_file_path = match args.pop() {
            Some(path) => path,
            None => return Err((String::from(MISSING_RAINBOW_TABLE_FILE_ARG), CRACK_HASH_PARSE_ERROR)),
        };
        Ok(CrackHashOptions {
            hash: hash,
            rainbow_table_file_path: rainbow_table_file_path,
        })
    }
}


impl fmt::Display for CrackHashOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "hash: {}, rainbow_table_file_path: {}", self.hash, self.rainbow_table_file_path)
    }
}


pub struct ProgramOptions {
    pub operation: AvailableOperations,
    // rainbow_table_file_path used for both generate_table and crack
    pub rainbow_table_file_path: String,
    // Options for generate_table
    pub word_file_path: Option<String>,
    // Options for crack
    pub hash: Option<String>,
}


impl ProgramOptions {

    pub fn new(mut args: Vec<String>) -> Result<ProgramOptions, (String, u8)> {
        let operation: AvailableOperations = match args.pop() {
            Some(operation_arg) => match &operation_arg[..] {
                GENERATE_TABLE_OPERATION => AvailableOperations::GenerateTable,
                CRACK_HASH_OPERATION => AvailableOperations::CrackHash,
                other => return Err((format!("Unknown operation {}", other), OPERATION_PARSE_ERROR_EXIT_CODE))
            },
            None => return Err((String::from(MISSING_OPERATION_ARG), OPERATION_PARSE_ERROR_EXIT_CODE))
        };
        // Obtain first cli arg after operation: rainbow table file path
        // since it is required for both generate_table and crack_hash
        let rainbow_table_file_path = match args.pop() {
            Some(path) => path,
            None => return Err((String::from(MISSING_RAINBOW_TABLE_FILE_ARG), ARGUMENT_PARSE_ERROR_EXIT_CODE)),
        };
        Ok(ProgramOptions {
            operation: operation,
            rainbow_table_file_path: rainbow_table_file_path,
            word_file_path: None,
            hash: None,
        })
    }

    pub fn get_generate_table_options(self) -> Result<GenerateTableOptions, (String, u8)> {
        let word_file_path = match self.word_file_path {
            Some(path) => path,
            None => {
                return Err((String::from(MISSING_WORD_FILE_ARG), GENERATE_TABLE_PARSE_ERROR));
            }
        };
        Ok(
            GenerateTableOptions {
                rainbow_table_file_path: self.rainbow_table_file_path,
                word_file_path: word_file_path,
            }
        )
    }

    pub fn get_crack_hash_options(self) -> Result<CrackHashOptions, (String, u8)> {
        let hash = match self.hash {
            Some(hash) => hash,
            None => {
                return Err((String::from(MISSING_HASH_ARG), CRACK_HASH_PARSE_ERROR));
            }
        };
        Ok(
            CrackHashOptions {
                rainbow_table_file_path: self.rainbow_table_file_path,
                hash: hash,
            }
        )
    }
}


fn print_available_operations() {
    eprintln!("Available Operations:");
    eprintln!("{}\n{}", GENERATE_TABLE_OPERATION, CRACK_HASH_OPEATION);
}

fn print_help() {
    let cli_args: Vec<String> = env::args().collect();
    let program_name = &cli_args[0];
    eprintln!("Usage: {} OPERATION [...operation_args]", program_name);
    print_available_operations();
}


pub fn parse_cli() -> ProgramOptions {
    let mut cli_args: Vec<String> = env::args().collect();
    if cli_args.len() <= 1 {
        print_help();
        process::exit(1);
    }
    // Reverse so we can pop from the first argument
    cli_args.reverse();
    cli_args.pop();
    ProgramOptions::new(cli_args).unwrap_or_else(|err| {
        let (err_msg, exit_code) = err;
        eprintln!("{}", err_msg);
        print_help();
        process::exit(exit_code.into());
    })
}
