use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process;

fn process_reader<R: BufRead>(reader: R, find: &str, replace: &str) {
    for line_result in reader.lines() {
        if let Ok(line) = line_result {
            let replaced = line.replace(find, replace);
            println!("{}", replaced);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Error: Please provide a find pattern and a replacement string.");
        eprintln!("Usage: <command> | rsed <find> <replace>   OR   rsed <find> <replace> [file]");
        process::exit(1);
    }

    let find = &args[1];
    let replace = &args[2];

    if args.len() > 3 {
        let file_path = &args[3];
        match File::open(file_path) {
            Ok(file) => process_reader(BufReader::new(file), find, replace),
            Err(e) => {
                eprintln!("Error opening file '{}': {}", file_path, e);
                process::exit(1);
            }
        }
    } else {
        let stdin = io::stdin();
        process_reader(stdin.lock(), find, replace);
    }
}
