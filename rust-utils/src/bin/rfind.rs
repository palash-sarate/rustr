use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn find_recursively(dir: &Path, query: &str) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                if file_name.contains(query) {
                    println!("{}", path.display());
                }
            }
            if path.is_dir() {
                find_recursively(&path, query);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: Please provide a search query.");
        eprintln!("Usage: rfind <query> [path]");
        process::exit(1);
    }

    let query = &args[1];
    let path_str = if args.len() > 2 { &args[2] } else { "." };
    let path = Path::new(path_str);

    if !path.exists() {
        eprintln!("Error: Path '{}' does not exist.", path_str);
        process::exit(1);
    }

    find_recursively(path, query);
}
