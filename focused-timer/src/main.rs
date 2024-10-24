use std::{thread, time::Duration};
use std::io::{self, Write};

#[derive(Debug)]
struct Task {
    description: String,
    priority: String,
    completed: bool,
}

impl Task {
    fn new(description: String, priority: String) -> Task {
        Task {
            description,
            priority,
            completed: false,
        }
    }

    fn mark_completed(&mut self) {
        self.completed = true;
    }
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        println!("=== Task Prioritizer & Focus Timer ===");
        println!("1. Add a Task");
        println!("2. List Tasks");
        println!("3. Mark Task as Completed");
        println!("4. Start Focus Timer (25 minutes)");
        println!("5. Exit");

        print!("Select an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                print!("Enter task description: ");
                io::stdout().flush().unwrap();
                let mut description = String::new();
                io::stdin().read_line(&mut description).unwrap();

                print!("Enter task priority (low, medium, high): ");
                io::stdout().flush().unwrap();
                let mut priority = String::new();
                io::stdin().read_line(&mut priority).unwrap();

                let task = Task::new(description.trim().to_string(), priority.trim().to_string());
                tasks.push(task);
                println!("Task added successfully!");
            }
            "2" => {
                println!("=== Task List ===");
                for (i, task) in tasks.iter().enumerate() {
                    println!(
                        "{}. {} [{}] - {}",
                        i + 1,
                        task.description,
                        task.priority,
                        if task.completed { "Completed" } else { "Pending" }
                    );
                }
            }
            "3" => {
                print!("Enter task number to mark as completed: ");
                io::stdout().flush().unwrap();
                let mut task_num = String::new();
                io::stdin().read_line(&mut task_num).unwrap();

                if let Ok(index) = task_num.trim().parse::<usize>() {
                    if let Some(task) = tasks.get_mut(index - 1) {
                        task.mark_completed();
                        println!("Task marked as completed!");
                    } else {
                        println!("Invalid task number.");
                    }
                } else {
                    println!("Please enter a valid number.");
                }
            }
            "4" => {
                println!("Starting Focus Timer for 25 minutes...");
                let duration = Duration::from_secs(25 * 60);
                thread::sleep(duration);
                println!("Focus session complete! Take a break.");
            }
            "5" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
        println!();
    }
}
