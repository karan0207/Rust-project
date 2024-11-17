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