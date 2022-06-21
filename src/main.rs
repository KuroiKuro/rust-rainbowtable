use rust_rainbowtable::operations::{HashCracker, Operator, RainbowTableGenerator};
use std::process::exit;

use clap::{Parser, Subcommand};

const RAINBOW_TABLE_ARG_HELP: &str = "Path to the rainbow table file";
const WORD_FILE_ARG_HELP: &str = "Path to the word file";
const HASH_ARG_HELP: &str = "Hash to crack";
const THREADS_ARG_HELP: &str = "Number of threads";
const DEFAULT_THREAD_COUNT: u32 = 1;

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
    #[clap(short = 't', long = "threads", help = THREADS_ARG_HELP)]
    pub threads: Option<u32>,
}

fn main() {
    let args = Cli::parse();
    let operator: Box<dyn Operator> = match args.command {
        Commands::CrackHash {
            rainbow_table_file_path,
            hash,
        } => Box::new(HashCracker::new(rainbow_table_file_path, hash)),
        Commands::GenerateTable {
            rainbow_table_file_path,
            word_file_path,
        } => Box::new(RainbowTableGenerator::new(
            word_file_path,
            rainbow_table_file_path,
        )),
    };
    let _threads = match args.threads {
        Some(threads) => threads,
        None => DEFAULT_THREAD_COUNT
    };
    let exit_code = operator.run();
    exit(exit_code);
}
