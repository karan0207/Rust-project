use clap::{Arg, Command};
use serde::Deserialize;
use std::fs;
use tokio::process::Command as TokioCommand;
use tracing::{info, Level};
use tracing_subscriber;

#[derive(Debug, Deserialize)]
struct Task {
    name: String,
    command: String,
    depends_on: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct Workflow {
    tasks: Vec<Task>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    // Parse CLI arguments
    let matches = Command::new("Task Orchestrator")
        .version("0.1.0")
        .about("An intelligent CLI Task Orchestrator")
        .arg(
            Arg::new("workflow")
                .short('w')
                .long("workflow")
                .takes_value(true)
                .help("Path to the workflow YAML file"),
        )
        .get_matches();

    let workflow_path = matches
        .value_of("workflow")
        .ok_or_else(|| anyhow::anyhow!("Workflow file is required"))?;

    // Load and parse the workflow
    let workflow_data = fs::read_to_string(workflow_path)?;
    let workflow: Workflow = serde_yaml::from_str(&workflow_data)?;

    // Execute the workflow
    execute_workflow(workflow).await?;

    Ok(())



}


async fn execute_workflow(workflow: Workflow) -> anyhow::Result<()> {
    info!("Starting workflow execution...");

    for task in &workflow.tasks {
        info!("Executing task: {}", task.name);
        let output = TokioCommand::new("sh")
            .arg("-c")
            .arg(&task.command)
            .output()
            .await?;

        if output.status.success() {
            info!(
                "Task '{}' completed successfully",
                task.name
            );
        } else {
            tracing::error!(
                "Task '{}' failed with error: {:?}",
                task.name,
                String::from_utf8_lossy(&output.stderr)
            );
            return Err(anyhow::anyhow!("Task '{}' failed", task.name));
        }
    }

    info!("Workflow execution completed.");
    Ok(())
}