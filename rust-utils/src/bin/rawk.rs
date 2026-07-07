use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process;

fn process_reader<R: BufRead>(reader: R, cols: &[usize], delimiter: Option<&str>) {
    for line_result in reader.lines() {
        if let Ok(line) = line_result {
            let parts: Vec<&str> = match delimiter {
                Some(delim) => line.split(delim).collect(),
                None => line.split_whitespace().collect(),
            };

            let mut output = Vec::new();
            for &col in cols {
                // col is 1-based index from user
                if col > 0 && col <= parts.len() {
                    output.push(parts[col - 1]);
                } else {
                    output.push("");
                }
            }
            println!("{}", output.join(" "));
        }
    }
}

fn parse_columns(cols_str: &str) -> Vec<usize> {
    cols_str
        .split(',')
        .map(|s| s.trim().parse::<usize>().unwrap_or(0))
        .filter(|&n| n > 0)
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: Please specify the columns to print (comma-separated, 1-based).");
        eprintln!("Usage: <command> | rawk <cols> [delimiter] [file]");
        eprintln!("Example: rawk 1,3 \",\" data.txt");
        process::exit(1);
    }

    let cols = parse_columns(&args[1]);
    if cols.is_empty() {
        eprintln!("Error: No valid column indexes provided.");
        process::exit(1);
    }

    let mut delimiter = None;
    let mut file_path = None;

    if args.len() > 2 {
        delimiter = Some(args[2].as_str());
    }

    if args.len() > 3 {
        file_path = Some(&args[3]);
    }

    // If delimiter is empty string, treat it as None (whitespace splitting)
    if let Some("") = delimiter {
        delimiter = None;
    }

    if let Some(path) = file_path {
        match File::open(path) {
            Ok(file) => process_reader(BufReader::new(file), &cols, delimiter),
            Err(e) => {
                eprintln!("Error opening file '{}': {}", path, e);
                process::exit(1);
            }
        }
    } else {
        let stdin = io::stdin();
        process_reader(stdin.lock(), &cols, delimiter);
    }
}
