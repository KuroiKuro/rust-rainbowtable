/*
    This module handles the parsing of CLI options.
*/
use std::fmt;
use clap::{Parser, Subcommand};

const RAINBOW_TABLE_ARG_HELP: &str = "Path to the rainbow table file";
const WORD_FILE_ARG_HELP: &str = "Path to the word file";
const HASH_ARG_HELP: &str = "Hash to crack";

#[derive(Subcommand)]
pub enum Commands {
    GenerateTable {
        #[clap(short = 'r', long = "rainbow-table-file", help = RAINBOW_TABLE_ARG_HELP)]
        rainbow_table_file_path: String,
        #[clap(short = 'w', long = "word-file", help = WORD_FILE_ARG_HELP)]
        word_file_path: String,
    },
    CrackHash {
        #[clap(short = 'r', long = "rainbow-table-file", help = RAINBOW_TABLE_ARG_HELP)]
        rainbow_table_file_path: String,
        #[clap(short = 'H', long = "hash", help = HASH_ARG_HELP)]
        hash: String,
    }
}

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands
}


pub struct GenerateTableOptions {
    pub word_file_path: String,
    pub rainbow_table_file_path: String,
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


impl fmt::Display for CrackHashOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "hash: {}, rainbow_table_file_path: {}", self.hash, self.rainbow_table_file_path)
    }
}


pub struct ProgramOptions {
    pub operation: Commands,
}

impl ProgramOptions {

    pub fn new() -> ProgramOptions {
        let args = Cli::parse();
        ProgramOptions {
            operation: args.command
        }
    }

    pub fn get_generate_table_options(&mut self) -> Option<GenerateTableOptions> {
        match &self.operation {
            Commands::GenerateTable { rainbow_table_file_path, word_file_path } => {
                Some(GenerateTableOptions {
                    rainbow_table_file_path: rainbow_table_file_path.to_string(),
                    word_file_path: word_file_path.to_string(),
                })
            },
            _ => None
        }
    }

    pub fn get_crack_hash_options(&mut self) -> Option<CrackHashOptions> {
        match &self.operation {
            Commands::CrackHash { rainbow_table_file_path, hash } => {
                Some(CrackHashOptions {
                    rainbow_table_file_path: rainbow_table_file_path.to_string(),
                    hash: hash.to_string(),
                })
            },
            _ => None
        }
    }
}
