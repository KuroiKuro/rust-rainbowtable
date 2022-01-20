/*
    This module handles the parsing of CLI options.
*/
use std::env;
use std::process;

enum AvailableOperations {
    generate_table,
    crack,
}

struct ProgramOptions<O> {
    operation: AvailableOperations,
    operation_options: O,
}

struct GenerateTableOptions {
    word_file: str,
}

fn print_help() {
    let program_name = env::args.get(0);
    eprintln!("Usage: {} OPERATION [...operation_args]", program_name);
    eprintln!("Available Operations:")
    eprintln!("generate_table")
}

pub fn parse_cli() -> ProgramOptions {
    let cli_args: Vec<String> = env::args().collect();
    if (cli_args.len() == 1) {
        print_help();
        process::exit(1);
    }
}
