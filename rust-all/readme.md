Explanation of the Code:
Enum (TaskStatus): This defines the possible states for a task (either Pending or Completed). This helps in managing task statuses.

Struct (Task): The Task struct holds the name of the task and its current status, which allows us to create a task with a specific name and status.

Methods (Task::new and Task::complete): These methods are used to create a new task and change the status of a task to Completed.

File I/O (read_tasks_from_file and save_tasks_to_file): The code reads tasks from a file when the program starts and saves them back to the file after modifications. This allows data persistence across program runs.

Concurrency (Arc<Mutex<T>>): The Arc<Mutex<T>> combination ensures safe shared ownership of the task list across multiple threads. This is used to safely read and write tasks concurrently.

Control Flow (match and if): Rust’s match statement is used to check the status of a task, while if statements are used to perform logic based on user input.

User Input and Interaction: The program interacts with the user through standard input, asking for task names and task numbers for completion, and then displays the updated task list.