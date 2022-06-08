use rust_rainbowtable::cli::ProgramOptions;
use rust_rainbowtable::operations;
use std::process::exit;

fn main() {
    let program_options = ProgramOptions::new();
    let exit_code = operations::select_run(program_options);
    exit(exit_code);
}
