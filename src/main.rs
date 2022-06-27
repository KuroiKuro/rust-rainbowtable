use rust_rainbowtable::operations::{HashCracker, Operator, RainbowTableGenerator};
use std::process::exit;

use clap::{Parser, Subcommand};

const RAINBOW_TABLE_ARG_HELP: &str = "Path to the rainbow table file";
const WORD_FILE_ARG_HELP: &str = "Path to the word file";
const HASH_ARG_HELP: &str = "Hash to crack";
const THREADS_ARG_HELP: &str = "Number of threads";
const DEFAULT_THREAD_COUNT: u32 = 1;

const THREAD_LIMIT: u32 = 500;
const THREAD_LIMIT_EXCEEDED: &str = "Exceeded maximum number of threads: 500";
const THREAD_LIMIT_EXCEEDED_EXIT_CODE: i32 = 1;

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
    pub threads: Option<usize>,
}

fn main() {
    let args = Cli::parse();
    let threads = match args.threads {
        Some(threads) => threads,
        None => DEFAULT_THREAD_COUNT,
    };

    // Safety feature
    if threads > THREAD_LIMIT {
        exit(THREAD_LIMIT_EXCEEDED_EXIT_CODE);
    }

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
            threads,
        )),
    };

    let exit_code = operator.run();
    exit(exit_code);
}
