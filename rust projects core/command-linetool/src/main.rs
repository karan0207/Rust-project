use clap::Parser;
use std::fs;
use std::io::{self, BufRead};

/// Struct to parse CLI arguments
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the file to analyze
    #[arg(short, long)]
    file: String,
}

fn main() -> io::Result<()> {
    // Parse the CLI arguments
    let args = Cli::parse();
    
    // Read the file content
    let file_content = fs::read_to_string(&args.file)?;
    
    // Count lines, words, and characters
    let line_count = file_content.lines().count();
    let word_count = file_content.split_whitespace().count();
    let char_count = file_content.chars().count();
    
    // Print the counts
    println!("Lines: {}", line_count);
    println!("Words: {}", word_count);
    println!("Characters: {}", char_count);
    
    Ok(())
}
