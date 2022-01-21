use std::env;

mod cli;

fn get_filename() -> String {
    let args: Vec<String> = env::args().collect();
    let filename_str = args.get(1);
    match filename_str {
        Some(filename) => String::from(filename),
        None => String::new(),
    };
    return String::from(&args[1]);
}


fn main() {
    let program_options = cli::parse_cli();
    println!("Program options:");
    println!("Operation: {}", program_options.operation);
    println!("Options: {}", program_options.operation_options.word_file);
}
