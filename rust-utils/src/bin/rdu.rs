use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn get_dir_size(path: &Path) -> u64 {
    let mut total_size = 0;
    if let Ok(metadata) = fs::symlink_metadata(path) {
        if metadata.is_file() {
            total_size += metadata.len();
        } else if metadata.is_dir() {
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries.flatten() {
                    total_size += get_dir_size(&entry.path());
                }
            }
        }
    }
    total_size
}

fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let target_str = if args.len() > 1 { &args[1] } else { "." };
    let target_path = Path::new(target_str);

    if !target_path.exists() {
        eprintln!("Error: Path '{}' does not exist.", target_str);
        process::exit(1);
    }

    if target_path.is_file() {
        if let Ok(meta) = fs::metadata(target_path) {
            println!("{}\t{}", format_size(meta.len()), target_str);
        }
    } else {
        // List sizes of immediate children
        if let Ok(entries) = fs::read_dir(target_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                let size = get_dir_size(&path);
                let name = path.file_name().unwrap_or_default().to_string_lossy();
                println!("{}\t{}", format_size(size), name);
            }
        }
        // Print total size
        let total = get_dir_size(target_path);
        println!("--------------------------------");
        println!("Total:\t{}", format_size(total));
    }
}
