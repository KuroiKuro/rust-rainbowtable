mod cli;
mod reader;
mod hasher;
mod operations;

use cli::ProgramOptions;

fn main() {
    let program_options = ProgramOptions::new();
    operations::select_run(program_options);
}
