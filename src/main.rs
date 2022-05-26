use rust_rainbowtable::operations;
use rust_rainbowtable::cli::ProgramOptions;

fn main() {
    let program_options = ProgramOptions::new();
    operations::select_run(program_options);
}
