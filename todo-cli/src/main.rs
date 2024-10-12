use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufReader, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    done: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct ToDoList {
    tasks: Vec<Task>,
}

impl ToDoList {
    fn new() -> Self {
        ToDoList { tasks: vec![] }
    }

    fn add_task(&mut self, description: String) {
        self.tasks.push(Task {
            description,
            done: false,
        });
        println!("Task added!");
    }

    fn list_tasks(&self) {
        for (index, task) in self.tasks.iter().enumerate() {
            let status = if task.done { "[Done]" } else { "[Pending]" };
            println!("{}: {} {}", index + 1, task.description, status);
        }
    }

    fn remove_task(&mut self, index: usize) {
        if index > 0 && index <= self.tasks.len() {
            self.tasks.remove(index - 1);
            println!("Task removed!");
        } else {
            println!("Invalid task number!");
        }
    }

    fn mark_done(&mut self, index: usize) {
        if index > 0 && index <= self.tasks.len() {
            self.tasks[index - 1].done = true;
            println!("Task marked as done!");
        } else {
            println!("Invalid task number!");
        }
    }

    fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let file = File::create(filename)?;
        serde_json::to_writer(file, &self.tasks)?;
        Ok(())
    }

    fn load_from_file(filename: &str) -> io::Result<Self> {
        if Path::new(filename).exists() {
            let file = File::open(filename)?;
            let reader = BufReader::new(file);
            let tasks: Vec<Task> = serde_json::from_reader(reader)?;
            Ok(ToDoList { tasks })
        } else {
            Ok(ToDoList::new())
        }
    }
}

fn main() {
    let mut todo_list = ToDoList::load_from_file("todo.json").unwrap_or_else(|_| ToDoList::new());

    loop {
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Remove Task");
        println!("4. Mark Task Done");
        println!("5. Save and Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice: u32 = choice.trim().parse().expect("Invalid input");

        match choice {
            1 => {
                println!("Enter task description:");
                let mut description = String::new();
                io::stdin().read_line(&mut description).expect("Failed to read input");
                todo_list.add_task(description.trim().to_string());
            }
            2 => {
                todo_list.list_tasks();
            }
            3 => {
                println!("Enter task number to remove:");
                let mut index = String::new();
                io::stdin().read_line(&mut index).expect("Failed to read input");
                let index: usize = index.trim().parse().expect("Invalid input");
                todo_list.remove_task(index);
            }
            4 => {
                println!("Enter task number to mark as done:");
                let mut index = String::new();
                io::stdin().read_line(&mut index).expect("Failed to read input");
                let index: usize = index.trim().parse().expect("Invalid input");
                todo_list.mark_done(index);
            }
            5 => {
                todo_list.save_to_file("todo.json").expect("Failed to save tasks");
                println!("Tasks saved. Exiting...");
                break;
            }
            _ => {
                println!("Invalid option. Please try again.");
            }
        }
    }
}
