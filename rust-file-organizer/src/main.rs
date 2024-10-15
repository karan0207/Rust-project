use std::env;
use std::fs;
use std::path::Path;
use std::collections::HashMap;

fn main() {
    // Get the directory from the command-line arguments
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: rust-file-organizer <directory>");
        std::process::exit(1);
    }

    let dir = &args[1];
    
    if !Path::new(dir).exists() {
        eprintln!("Directory '{}' does not exist.", dir);
        std::process::exit(1);
    }

    organize_files_by_extension(dir);
}

fn organize_files_by_extension(dir: &str) {
    let mut file_map: HashMap<String, Vec<String>> = HashMap::new();

    // Read the directory and categorize files by extension
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            if let Some(extension) = path.extension() {
                let ext = extension.to_string_lossy().to_string();
                let file_name = path.file_name().unwrap().to_string_lossy().to_string();

                file_map.entry(ext).or_insert_with(Vec::new).push(file_name);
            }
        }
    }

    // Print the organized files
    for (ext, files) in &file_map {
        println!("\nFiles with extension '{}':", ext);
        for file in files {
            println!("  - {}", file);
        }
    }
}
