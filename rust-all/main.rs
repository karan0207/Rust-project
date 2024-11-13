use std::fs::{File, OpenOptions};
use std::io::{self, Write, BufReader, BufRead};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// Enum to represent the status of the task
enum TaskStatus {
    Pending,
    Completed,
}

// Struct to represent a task
struct Task {
    name: String,
    status: TaskStatus,
}

// Implementation block for Task
impl Task {
    // Constructor method to create a new task with Pending status
    fn new(name: String) -> Task {
        Task {
            name,
            status: TaskStatus::Pending,
        }
    }

    // Method to complete the task by changing its status to Completed
    fn complete(&mut self) {
        self.status = TaskStatus::Completed;
    }
}


// Function to read tasks from a file
fn read_tasks_from_file(file_name: &str) -> io::Result<Vec<Task>> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    let mut tasks = Vec::new();

    // Read each line from the file and create a task
    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split(',');
        let name = parts.next().unwrap_or_default().to_string();
        let status_str = parts.next().unwrap_or_default();
        let status = match status_str {
            "Completed" => TaskStatus::Completed,
            _ => TaskStatus::Pending,
        };

        tasks.push(Task { name, status });
    }

    Ok(tasks)
}


// Function to save tasks to a file
fn save_tasks_to_file(file_name: &str, tasks: &Vec<Task>) -> io::Result<()> {
    let file = OpenOptions::new().create(true).write(true).truncate(true).open(file_name)?;
    for task in tasks {
        let status = match task.status {
            TaskStatus::Completed => "Completed",
            TaskStatus::Pending => "Pending",
        };
        writeln!(file, "{},{}", task.name, status)?;
    }
    Ok(())
}



// Function to list tasks and display their status
fn list_tasks(tasks: &Vec<Task>) {
    for (index, task) in tasks.iter().enumerate() {
        let status = match &task.status {
            TaskStatus::Completed => "Completed",
            TaskStatus::Pending => "Pending",
        };
        println!("{}. {} - {}", index + 1, task.name, status);
    }
}
