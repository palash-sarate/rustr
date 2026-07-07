use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let target_dir = if args.len() > 1 {
        &args[1]
    } else {
        "."
    };

    match fs::read_dir(target_dir) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_name = entry.file_name();
                    if let Some(name_str) = file_name.to_str() {
                        println!("{}", name_str);
                    }
                }
            }
        }
        Err(error) => {
            eprintln!("Error reading directory '{}': {}", target_dir, error);
            process::exit(1);
        }
    }
}
