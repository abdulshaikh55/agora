use std::io::Write; // used by .flush() in add_task funtion

// Abdul, let's start with creating a function that reads the task from the user.
#[allow(dead_code)]
fn add_task() -> String{
    let mut task:String = String::new();
    print!("➕Add task: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut task).expect("Failed to read input");
    let task: String = task.trim().parse().ok().expect("Enter a valid input");
    task
}

// Now, let's create a funtion that displays all the tasks
#[allow(dead_code)]
fn display_all(tasks: Vec<String>) {
    for (idx, task) in tasks.iter().enumerate() {
        println!("{} -> {}", idx, *task); 
    }
}

fn main() {
    // a vector type DS that will hold all the tasks.
    let mut tasks: Vec<String> = Vec::new();
    
    println!("Hello User! How can I help you?");
    loop {
        print!("******************\n1. Add Task\n2. Delete Task\n3. Display list\n4. Exit\nChoose an option: ");
        std::io::stdout().flush().unwrap();
        let mut choice: String = String::new();
        std::io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice: u8 = choice.trim().parse().expect("Enter a valid input");
        if !(1..=4).contains(&choice) {
            println!("❌Invalid input. Try again");
            continue; // Jump back to the start of the loop
        }
        match choice {
            1 => { // adds new task to the list
                let new_task: String = add_task();
                tasks.push(new_task);
                println!("✅Task list updated.");
            },

            2 => { // to delete a specific task or all the tasks

            },

            3 => { // To display all the tasks
                display_all(tasks.clone())
            },

            4 => { // ro exit the program
                break;
            },

            _ => (),
        }
    }
    
}