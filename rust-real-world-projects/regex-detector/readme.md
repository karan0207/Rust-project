Regex Detector in Rust
This Rust project demonstrates how to use regular expressions (regex) to detect various patterns in text, using the regex crate. The project includes different types of regex patterns to match common string formats, such as email addresses, URLs, phone numbers, and more.

Features
Basic Regex Matching: Matches simple patterns like digits, letters, and whitespace.
Email Validator: Matches email addresses.
URL Validator: Matches URLs.
Phone Number Validator: Matches phone numbers in different formats.
Date Validator: Matches dates in various formats.
Custom Pattern Matching: Demonstrates the power of custom regex patterns for more advanced use cases.
Prerequisites
Before running the project, make sure you have the following installed:

Rust: The programming language used for this project.
Cargo: Rust's package manager and build tool.
Installation
Clone the repository:

bash
Copy code
git clone https://github.com/your-username/regex-detector-rust.git
cd regex-detector-rust
Build the project:

bash
Copy code
cargo build
Run the project:

bash
Copy code
cargo run
Usage
After running the project, the program will test different types of regular expressions and print the results based on whether the given strings match the respective patterns.

Types of Regular Expressions Used:
Email Validation Regex:

Pattern: Validates email addresses with standard formats (e.g., user@example.com).
Example: user@example.com.
URL Validation Regex:

Pattern: Validates URLs starting with http, https, or ftp schemes.
Example: https://www.example.com.
Phone Number Validation Regex:

Pattern: Validates phone numbers in formats like (123) 456-7890 or 123-456-7890.
Example: (123) 456-7890.
Date Validation Regex:

Pattern: Matches dates in the format DD/MM/YYYY or MM-DD-YYYY.
Example: 15/08/2024 or 08-15-2024.