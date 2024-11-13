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