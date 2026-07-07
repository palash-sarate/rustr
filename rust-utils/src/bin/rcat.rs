use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: Please provide a file path.");
        eprintln!("Usage: rcat <file_path>");
        process::exit(1);
    }

    let file_path = &args[1];

    match fs::read_to_string(file_path) {
        Ok(contents) => {
            print!("{}", contents);
        }
        Err(error) => {
            eprintln!("Error reading '{}': {}", file_path, error);
            process::exit(1);
        }
    }
}
