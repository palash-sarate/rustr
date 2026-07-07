use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process;

fn fuzzy_match(line: &str, query: &str) -> bool {
    let line_chars: Vec<char> = line.to_lowercase().chars().collect();
    let query_chars: Vec<char> = query.to_lowercase().chars().collect();

    let mut line_idx = 0;
    let mut query_idx = 0;

    while line_idx < line_chars.len() && query_idx < query_chars.len() {
        if line_chars[line_idx] == query_chars[query_idx] {
            query_idx += 1;
        }
        line_idx += 1;
    }

    query_idx == query_chars.len()
}

fn process_reader<R: BufRead>(reader: R, query: &str) {
    for line_result in reader.lines() {
        if let Ok(line) = line_result {
            if fuzzy_match(&line, query) {
                println!("{}", line);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: Please provide a fuzzy query.");
        eprintln!("Usage: <command> | rfzf <query>   OR   rfzf <query> [file]");
        process::exit(1);
    }

    let query = &args[1];

    if args.len() > 2 {
        let file_path = &args[2];
        match File::open(file_path) {
            Ok(file) => process_reader(BufReader::new(file), query),
            Err(e) => {
                eprintln!("Error opening file '{}': {}", file_path, e);
                process::exit(1);
            }
        }
    } else {
        let stdin = io::stdin();
        process_reader(stdin.lock(), query);
    }
}
