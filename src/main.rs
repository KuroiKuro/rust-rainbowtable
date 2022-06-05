use std::process::exit;
use rust_rainbowtable::operations;
use rust_rainbowtable::cli::ProgramOptions;

fn main() {
    let program_options = ProgramOptions::new();
    let exit_code = operations::select_run(program_options);
    exit(exit_code);
}
