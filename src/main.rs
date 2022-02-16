mod cli;
mod reader;
mod hasher;
mod operations;


fn main() {
    let program_options = cli::parse_cli();
    operations::select_run(program_options);
}
