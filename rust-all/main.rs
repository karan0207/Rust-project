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