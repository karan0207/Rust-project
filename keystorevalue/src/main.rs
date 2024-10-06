use std::collections::HashMap;
use std::io;

fn main() {
    let mut store: HashMap<String, String> = HashMap::new();
    loop {
        println!("Welcome to the Key-Value Store!");
        println!("1. Add key-value pair");
        println!("2. Get value by key");
        println!("3. Delete key-value pair");
        println!("4. List all key-value pairs");
        println!("5. Exit");
        println!("Choose an option: ");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = choice.trim().parse().expect("Please enter a number");

        match choice {
            1 => {
                println!("Enter key: ");
                let mut key = String::new();
                io::stdin().read_line(&mut key).expect("Failed to read line");
                let key = key.trim().to_string();

                println!("Enter value: ");
                let mut value = String::new();
                io::stdin().read_line(&mut value).expect("Failed to read line");
                let value = value.trim().to_string();

                store.insert(key, value);
                println!("Key-value pair added.");
            },
            2 => {
                println!("Enter key to retrieve: ");
                let mut key = String::new();
                io::stdin().read_line(&mut key).expect("Failed to read line");
                let key = key.trim();

                match store.get(key) {
                    Some(value) => println!("Value: {}", value),
                    None => println!("Key not found."),
                }
            },
            3 => {
                println!("Enter key to delete: ");
                let mut key = String::new();
                io::stdin().read_line(&mut key).expect("Failed to read line");
                let key = key.trim();

                if store.remove(key).is_some() {
                    println!("Key-value pair deleted.");
                } else {
                    println!("Key not found.");
                }
            },
            4 => {
                if store.is_empty() {
                    println!("No key-value pairs stored.");
                } else {
                    for (key, value) in &store {
                        println!("{}: {}", key, value);
                    }
                }
            },
            5 => {
                println!("Exiting...");
                break;
            },
            _ => println!("Invalid option! Please try again."),
        }
    }
}
