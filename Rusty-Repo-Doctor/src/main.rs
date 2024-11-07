use git2::Repository;
use cargo_metadata::{MetadataCommand, Dependency};
use std::path::Path;
use serde::Serialize;

#[derive(Serialize)]
struct DependencyInfo {
    name: String,
    version: String,
    is_outdated: bool,
}

fn main() {
    // Get repository info
    match analyze_git_repo(".") {
        Ok(_) => println!("Repository analysis complete."),
        Err(e) => eprintln!("Error analyzing repository: {:?}", e),
    }

    // Check dependencies
    match check_dependencies() {
        Ok(dependencies) => {
            println!("Dependencies Analysis:");
            for dep in dependencies {
                println!(
                    "Dependency: {} - Version: {} - Outdated: {}",
                    dep.name, dep.version, dep.is_outdated
                );
            }
        }
        Err(e) => eprintln!("Error checking dependencies: {:?}", e),
    }
}
