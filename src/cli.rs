/*
    This module handles the parsing of CLI options.
*/
use std::env;
use std::fmt;
use std::process;

const GENERATE_TABLE_OPERATION: &str = "generate_table";
const MISSING_OPERATION_ARG: &str = "Missing operation argument";
const MISSING_WORD_FILE_ARG: &str = "Missing argument word_file for generate_table operation!";
const MISSING_RAINBOW_TABLE_FILE_ARG: &str = "Missing argument rainbow_table_file for generate_table operation!";

const OPERATION_PARSE_ERROR_EXIT_CODE: u8 = 1;
const GENERATE_TABLE_PARSE_ERROR: u8 = 2;

pub enum AvailableOperations {
    GenerateTable,
    // Crack,
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

pub struct ProgramOptions {
    pub operation: AvailableOperations,
    pub operation_options: GenerateTableOptions,
}

impl ProgramOptions {

    pub fn new(mut args: Vec<String>) -> Result<ProgramOptions, (String, u8)> {
        let operation: AvailableOperations = match args.pop() {
            Some(operation_arg) => match &operation_arg[..] {
                GENERATE_TABLE_OPERATION => AvailableOperations::GenerateTable,
                other => return Err((format!("Unknown operation {}", other), OPERATION_PARSE_ERROR_EXIT_CODE))
            },
            None => return Err((String::from(MISSING_OPERATION_ARG), OPERATION_PARSE_ERROR_EXIT_CODE))
        };
    
        match operation {
            AvailableOperations::GenerateTable => {
                let opts = GenerateTableOptions::new(args)?;
                let parsed_options = ProgramOptions {
                    operation: AvailableOperations::GenerateTable,
                    operation_options: opts
                };
                Ok(parsed_options)
            }
        }
    }
}


fn print_available_operations() {
    eprintln!("Available Operations:");
    eprintln!("{}", GENERATE_TABLE_OPERATION);
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
    cli_args.reverse();
    cli_args.pop();
    ProgramOptions::new(cli_args).unwrap_or_else(|err| {
        let (err_msg, exit_code) = err;
        eprintln!("{}", err_msg);
        print_help();
        process::exit(exit_code.into());
    })
}
