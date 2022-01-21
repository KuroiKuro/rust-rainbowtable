/*
    This module handles the parsing of CLI options.
*/
use std::env;
use std::fmt;
use std::process;

const GENERATE_TABLE_OPERATION: &str = "generate_table";

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
    pub word_file: String,
}

impl fmt::Display for GenerateTableOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "word_file: {}", &self.word_file)
    }
}

pub struct ProgramOptions {
    pub operation: AvailableOperations,
    pub operation_options: GenerateTableOptions,
}

impl ProgramOptions {

    pub fn new(mut args: Vec<String>) -> Result<ProgramOptions, String> {
        let operation: AvailableOperations = match args.pop() {
            Some(operation_arg) => match &operation_arg[..] {
                GENERATE_TABLE_OPERATION => AvailableOperations::GenerateTable,
                other => return Err(format!("Unknown operation {}", other))
            },
            None => return Err(String::from("Missing operation argument"))
        };
    
        match operation {
            AvailableOperations::GenerateTable => {
                let opts = parse_generate_table_options(args);
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

fn parse_generate_table_options(mut args: Vec<String>) -> GenerateTableOptions {
    let word_file = match args.pop() {
        Some(wf) => wf,
        None => {
            eprintln!("Missing argument word_file for generate_table operation!");
            process::exit(2);
        }
    };
    GenerateTableOptions {
        word_file: word_file
    }
}

pub fn parse_cli() -> ProgramOptions {
    let cli_args: Vec<String> = env::args().collect();
    if cli_args.len() <= 1 {
        print_help();
        process::exit(1);
    }

    ProgramOptions::new(cli_args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        print_help();
        process::exit(1);
    })
}
