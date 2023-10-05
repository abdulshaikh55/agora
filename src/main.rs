use std::io::Write; // used by .flush() in add_task function
// Abdul, let's start with creating a function that reads the task from the user.
fn add_task() -> String {
    let mut task: String = String::new();
    print!("➕ Add task: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut task).expect("Failed to read input");
    let task: String = task.trim().parse().expect("Enter a valid input");
    task
}

// Now, let's create a function that displays all the tasks
fn display_all(tasks: Vec<String>) {
    for (idx, task) in tasks.iter().enumerate() {
        println!("{} -> {}", idx, *task); 
    }
}

// This function allows us to choose between deleting all, one, or latest task in the list.
fn delete(tasks: &mut Vec<String>) -> Option<String> {
    loop {
        let mut choice: String = String::new();
        println!("1. Delete last task\n2. Delete all tasks\n3. Delete specific task\n4. Cancel");
        std::io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice: u8 = choice.trim().parse().expect("Enter a valid input");
        match choice {
            1 => { // Deletes the latest task on the list
                if let Some(task) = tasks.pop() {
                    return Some(task);
                }
            },

            2 => { // Deletes all the tasks on the list
                tasks.clear();
                println!("All tasks deleted.");
            },

            3 => { // Deletes the string specified by the user
                let mut idx: String = String::new();
                print!("Enter the index of the task: ");
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut idx).expect("Failed to read input");
                let idx: usize = idx.trim().parse().expect("Enter a valid input");
                if let Some(task) = tasks.get(idx) {
                    let task: String = task.to_owned();
                    tasks.remove(idx);
                    return Some(task);
                } else {
                    println!("Invalid index.");
                }
            },

            4 => break,
            _ => {
                println!("Please select a valid choice.");
                continue;
            }
        }
    }
    None
}

fn main() {
    // a vector type DS that will hold all the tasks.
    let mut tasks: Vec<String> = Vec::new();

    println!("Hello User! How can I help you?");
    loop {
        print!("\n******************\n✅ 1. Add Task\n⛔ 2. Delete Task\n🫙 3. Display list\n📤 4. Exit\nChoose an option: ");
        std::io::stdout().flush().unwrap();
        let mut choice: String = String::new();
        std::io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice: u8 = choice.trim().parse().expect("Enter a valid input");

        if !(1..=4).contains(&choice) {
            println!("❌ Invalid input. Try again");
            continue; // Jump back to the start of the loop
        }
        match choice {
            1 => { // adds new task to the list
                let new_task: String = add_task();
                tasks.push(new_task);
                println!("✅ Task list updated.");
            },

            2 => { // to delete a specific task or all the tasks
                if let Some(task) = delete(&mut tasks) {
                    println!("Task:'{}' deleted.", task)
                }
            },

            3 => { // To display all the tasks
                display_all(tasks.clone());
            },

            4 => { // to exit the program
                break;
            },

            _ => (), 
        }
    }
}