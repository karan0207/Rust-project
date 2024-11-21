# Rusty Repo Doctor

**Rusty Repo Doctor** is a CLI tool designed to perform basic analysis on Rust projects. This tool provides a simple way to analyze Git repositories and check for outdated dependencies in `Cargo.toml`. It’s ideal for developers looking to maintain clean and manageable codebases.

## Features

- **Dependency Checker**: Scans dependencies listed in `Cargo.toml` and provides version information.
- **Repository Analysis**: Analyzes Git repository details, such as counting the total number of commits in `HEAD`.

This project is a work in progress and can be extended to include additional features, such as checking for outdated dependencies, code quality analysis, and more!

## Getting Started

### Prerequisites

To run **Rusty Repo Doctor**, you’ll need:

- **Rust** (version 1.56+ is recommended)
- **Cargo** (Rust's package manager)

### Installation

Clone the repository and navigate to the project directory:

```bash
git clone https://github.com/yourusername/rusty_repo_doctor.git
cd rusty_repo_doctor
