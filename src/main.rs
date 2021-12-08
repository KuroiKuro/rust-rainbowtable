use std::env;

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
    // Get file name from cli
    let filename = get_filename();
    println!("Filename = {}", filename);
}
