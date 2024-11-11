use regex::Regex;
use std::io::{self, Write};

fn main() {
    // Define different regex patterns for each type
    let patterns = [
        ("Email", r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"),
        ("URL", r"^(http://|https://)?(www\.)?[a-zA-Z0-9-]+\.[a-zA-Z]{2,}(/[a-zA-Z0-9#-]+/?)*$"),
        ("IP Address", r"^(?:[0-9]{1,3}\.){3}[0-9]{1,3}$"),
        ("Phone Number", r"^\+?[0-9]{1,3}?[-. ]?(\(?[0-9]{3}\)?[-. ]?)?[0-9]{3}[-. ]?[0-9]{4}$"),
        ("Date (YYYY-MM-DD)", r"^\d{4}-\d{2}-\d{2}$"),
        ("Social Security Number", r"^\d{3}-\d{2}-\d{4}$"),
    ];

    println!("Select the type of input to validate:");
    for (i, (name, _)) in patterns.iter().enumerate() {
        println!("{}: {}", i + 1, name);
    }

    print!("Enter your choice (1-{}): ", patterns.len());
    io::stdout().flush().unwrap();

    // Get user choice
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice = choice.trim().parse::<usize>().unwrap_or(0);

    // Check if choice is within range
    if choice < 1 || choice > patterns.len() {
        println!("Invalid choice. Please restart and select a valid option.");
        return;
    }

    // Retrieve the selected pattern
    let (pattern_name, pattern_str) = patterns[choice - 1];
    let re = Regex::new(pattern_str).expect("Invalid regex pattern");

    println!("Selected pattern: {}", pattern_name);

    // Prompt the user to enter a string
    print!("Enter a string to check: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();

    // Check if the input matches the selected regex pattern
    if re.is_match(input) {
        println!("The input matches the {} pattern.", pattern_name);
    } else {
        println!("The input does not match the {} pattern.", pattern_name);
    }
}
