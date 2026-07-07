use std::env;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process;

fn search_in_file(path: &Path, pattern: &str) {
    if let Ok(file) = File::open(path) {
        let reader = BufReader::new(file);
        for (line_num, line_result) in reader.lines().enumerate() {
            if let Ok(line) = line_result {
                if line.contains(pattern) {
                    println!("{}:{}: {}", path.display(), line_num + 1, line.trim());
                }
            }
        }
    }
}

fn search_recursively(dir: &Path, pattern: &str) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                search_recursively(&path, pattern);
            } else if path.is_file() {
                search_in_file(&path, pattern);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: Please provide a search pattern.");
        eprintln!("Usage: rgrep <pattern> [path]");
        process::exit(1);
    }

    let pattern = &args[1];
    let path_str = if args.len() > 2 { &args[2] } else { "." };
    let path = Path::new(path_str);

    if !path.exists() {
        eprintln!("Error: Path '{}' does not exist.", path_str);
        process::exit(1);
    }

    if path.is_file() {
        search_in_file(path, pattern);
    } else {
        search_recursively(path, pattern);
    }
}
