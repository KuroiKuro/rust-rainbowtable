use rust_rainbowtable::operations;
use std::process::exit;

use clap::{Parser, Subcommand};
use std::fmt;

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
    },
}

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

struct GenerateTableOptions {
    pub word_file_path: String,
    pub rainbow_table_file_path: String,
}

impl fmt::Display for GenerateTableOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "word_file: {}, rainbow_table_file: {}",
            &self.word_file_path, &self.rainbow_table_file_path
        )
    }
}

struct CrackHashOptions {
    pub hash: String,
    pub rainbow_table_file_path: String,
}

impl fmt::Display for CrackHashOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "hash: {}, rainbow_table_file_path: {}",
            self.hash, self.rainbow_table_file_path
        )
    }
}

fn main() {
    // let args = Cli::parse();
    // match args.command {
    //     Commands::CrackHash { rainbow_table_file_path, hash } => {
            
    //     }
    // }
    // let exit_code = operations::select_run(program_options);
    // exit(exit_code);
}
