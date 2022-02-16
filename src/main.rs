mod cli;
mod reader;
mod hasher;
mod operations;


fn main() {
    let program_options = cli::parse_cli();
    println!("Program options:");
    println!("Operation: {}", program_options.operation);
    operations::select_run(program_options);
}
