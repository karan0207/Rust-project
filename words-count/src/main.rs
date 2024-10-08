use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: wordcount <file>");
        return;
    }

    let file_path = &args[1];
    
    // Open the file and handle any errors
    if let Ok(lines) = read_lines(file_path) {
        let mut word_count = 0;
        
        // Loop through each line and count words
        for line in lines {
            if let Ok(text) = line {
                word_count += text.split_whitespace().count();
            }
        }

        println!("Word count: {}", word_count);
    } else {
        eprintln!("Could not read file: {}", file_path);
    }
}

// Helper function to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
