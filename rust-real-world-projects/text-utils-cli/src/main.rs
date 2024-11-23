// src/main.rs
use std::env;
use std::fs;
use std::process;

fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

fn count_lines(text: &str) -> usize {
    text.lines().count()
}

fn count_chars(text: &str) -> usize {
    text.chars().count()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: textutils <file_path> <mode>");
        eprintln!("Modes: words, lines, chars");
        process::exit(1);
    }

    let file_path = &args[1];
    let mode = &args[2];

    let content = fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("Failed to read file: {}", err);
        process::exit(1);
    });

    match mode.as_str() {
        "words" => println!("Word count: {}", count_words(&content)),
        "lines" => println!("Line count: {}", count_lines(&content)),
        "chars" => println!("Character count: {}", count_chars(&content)),
        _ => {
            eprintln!("Unknown mode: {}. Use 'words', 'lines', or 'chars'.", mode);
            process::exit(1);
        }
    }
}
