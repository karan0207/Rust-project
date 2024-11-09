use clap::Parser;

/// A simple CLI tool that greets the user.
#[derive(Parser)]
#[command(name = "greet_cli")]
#[command(author = "Your Name")]
#[command(version = "1.0")]
#[command(about = "A simple CLI tool to greet users", long_about = None)]
struct Cli {
    /// The name of the user to greet
    name: String,
}

fn main() {
    let cli = Cli::parse();

    // Greet the user
    println!("Hello, {}! Welcome to the CLI tool!", cli.name);
}
