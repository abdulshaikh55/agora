use std::io::{self, Write, Read}; // Write used by .flush() in add_task function
use std::fs::File;
use serde_json;
use serde_derive::{Serialize, Deserialize};
// let's start with creating a struct Task that holds the task and its additional information

#[derive(Debug, Serialize, Deserialize)]
enum Status { 
    NotStarted, Ongoing, Completed // Just like my to-do list: never started, always ongoing, never completed!
}

#[derive(Debug, Serialize, Deserialize)]
enum Priority {
    Urgent, Important, Normal
}

#[derive(Debug, Serialize, Deserialize)]
// A struct that holds information about individual tasks
struct Task {
    title: String,
    description: String,
    due_date: String,
    status: Status,
    priority: Priority
}

#[derive(Debug, Serialize, Deserialize)]
// A struct that holds the vector of tasks
struct TaskManager {
    tasks: Vec<Task>
}

// an implementation that helps us in interacting with all the tasks
impl TaskManager {

    // This function adds new Task in the vector
    fn add_new_task(&mut self) {
        let mut title = String::new();
        let mut description = String::new();
        let mut due_date = String::new();
        let mut priority = String::new();

        print!("Enter task title: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut title).expect("Failed to read input");
        let title: String = title.trim().parse().expect("Enter a valid title");

        print!("Enter task description: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut description).expect("Failed to read input");
        let description: String = description.trim().parse().expect("Enter a valid description");

        println!("Choose task priority: ");
        println!("u - Urgent\ni - Important\nn - Normal");
        std::io::stdin().read_line(&mut priority).expect("Faild to read input");
        let priority: char = priority.trim().parse().expect("Enter a valid choice");
        let prior = match priority {
            'u' => Priority::Urgent,
            'i' => Priority::Important,
            'n' => Priority::Normal,
            _ => panic!("Invalid choice"),
        };

        print!("Enter task due date(DD/MM/YY): ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut due_date).expect("Failed to read input");
        let due_date: String = due_date.trim().to_string().parse().expect("Enter a valid due date:");

        // insert all the information and put it in the new_task instance
        let new_task = Task {
            title,
            description,
            due_date,
            status: Status::NotStarted,
            priority: prior,
        };

        self.tasks.push(new_task);
        println!("\nTask successfully added!\n");
    }

    // This function displays all the tasks and the additional information in the memory
    fn display_all(&self) {
        for (idx, task) in self.tasks.iter().enumerate() {
            println!("\n**************************************************");
            println!("Priority: {:?}", task.priority);
            println!("{} -> {}", idx + 1, task.title);
            println!("Description: {}", task.description);
            println!("Due Date: {}", task.due_date);
            println!("Status: {:?}", task.status);
            println!("**************************************************");
        }
    }

    fn update_task(&mut self) {
        loop {
            // first display all the tasks
            self.display_all();

            print!("Choose the task you want to update: ");
            std::io::stdout().flush().unwrap();
            let mut choice: String = String::new();
            std::io::stdin().read_line(&mut choice).expect("Failed to read input");
            let choice: usize = choice.trim().parse().expect("Enter a valid input");

            // check if task chosen to be updated exists in the list
            if choice > self.tasks.len() {
                println!("The task at that index does not exist. Try again.");
                continue;
            }
            if choice == 0 {
                break;
            }

            match self.tasks.get_mut(choice - 1) {
                Some(task) => {
                    println!("What do you want to modify?");
                    println!("1. Title");
                    println!("2. Description");
                    println!("3. Priority");
                    println!("4. Due Date");
                    println!("5. Status");
                    print!("6. Exit\n=> ");
                    std::io::stdout().flush().unwrap();
                    let mut modify_choice = String::new();
                    std::io::stdin().read_line(&mut modify_choice).expect("Failed to read input");
                    let modify_choice: u8 = modify_choice.trim().parse().expect("Enter a valid input");

                    match modify_choice {
                        1 => {
                            let mut title = String::new();
                            print!("Enter new title: ");
                            std::io::stdout().flush().unwrap();
                            std::io::stdin().read_line(&mut title).expect("Failed to read the title");
                            task.title = title.trim().to_string();
                        },
                        2 => {
                            let mut description = String::new();
                            print!("Enter new description: ");
                            std::io::stdout().flush().unwrap();
                            std::io::stdin().read_line(&mut description).expect("Failed to read description");
                            task.description = description.trim().to_string();
                        },
                        3 => {
                            let mut priority = String::new();
                            println!("Enter new status:\n'u'- Urgent\n'i' - Important\n'n' - Normal");
                            std::io::stdin().read_line(&mut priority).expect("failed to read priority");
                            let priority: char = priority.trim().parse().expect("Enter a valid choice");
                            let prior = match priority {
                                'u' => Priority::Urgent,
                                'i' => Priority::Important,
                                'n' => Priority::Normal,
                                _ => panic!("Invalid choice"),
                            };
                            task.priority = prior;
                        },
                        4 => {
                            let mut due_date = String::new();
                            print!("Enter new due date(DD/MM/YY): ");
                            std::io::stdout().flush().unwrap();
                            std::io::stdin().read_line(&mut due_date).expect("Failed to read due_date");
                            task.due_date = due_date.trim().to_string();
                        },
                        5 => {
                            let mut status = String::new();
                            print!("Enter new status: \n1. Not started\n2. Ongoing\n3. Completed\n => ");
                            std::io::stdout().flush().unwrap();
                            std::io::stdin().read_line(&mut status).expect("Failed to read status");
                            let status = match status.trim().parse() {
                                Ok(1) => Status::NotStarted,
                                Ok(2) => Status::Ongoing,
                                Ok(3) => Status::Completed,
                                _ => {
                                    println!("Invalid status");
                                    Status::NotStarted
                                }
                            };
                            task.status = status;
                        },
                        6 => break, // Exit the loop 
                        _ => {
                            println!("Invalid choice"); // if user does not choose the valid option
                            continue;
                        },
                    }
                },
                None => println!("There is no Task with that index")
            }
        }
    }

    // This function allows us to choose between deleting all, one, or latest task in the list.
    fn delete(&mut self) -> Option<Task> {
        loop {
            let mut choice: String = String::new();
            println!("1. Delete last task\n2. Delete all tasks\n3. Delete specific task\n4. Cancel");
            std::io::stdin().read_line(&mut choice).expect("Failed to read input");
            let choice: u8 = choice.trim().parse().expect("Enter a valid input");
            match choice {
                1 => { // Deletes the latest task on the list
                    if let Some(task) = self.tasks.pop() {
                        return Some(task);
                    }
                },
                2 => { // Deletes all the tasks on the list
                    self.tasks.clear();
                    println!("All tasks deleted.");
                },
                3 => { // Deletes the task specified by the user
                    let mut idx: String = String::new();
                    print!("Enter the index of the task: ");
                    std::io::stdout().flush().unwrap();
                    std::io::stdin().read_line(&mut idx).expect("Failed to read input");
                    let idx: usize = idx.trim().parse().expect("Enter a valid input");
                    let task = self.tasks.remove(idx);
                    return Some(task);
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

    fn save_to_file(&self, filename: &str) -> io::Result<()> {
        // #1 put the tasks in 'data' variable in serialized form using serde::json.
        let data = serde_json::to_string(&self)?;
        // #2 create a file and name it 'filename'. don't worry if the file is already there; it will just open the file with that name
        let mut file = File::create(filename)?;
        // #3 insert all the data into the file.
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    fn load_from_file(filename: &str) -> io::Result<TaskManager> {
        // #1 open the file you want to access and put it on file.
        let mut file = File::open(filename)?;
        // #2 initialize a variable to hold the content of that file.
        let mut file_content = String::new();
        // #3 insert the content into the variable.
        file.read_to_string(&mut file_content)?;

        // #4 deserialize the content and return it
        let tasks: TaskManager = serde_json::from_str(&file_content)?;
        Ok(tasks)
    }
}

fn main() {
    // a vector type DS that will hold all the tasks.
    let mut tasks: TaskManager = TaskManager::load_from_file("tasks.json").unwrap_or_else(|_| TaskManager { tasks: Vec::new() });

    println!("Hello User! How can I help you?");
    loop {
        print!("\n******************************\n✅ 1. Add Task\n➕ 2. Update Task\n⛔ 3. Delete Task\n🫙 4. Display list\n📤 5. Exit\nChoose an option => ");
        std::io::stdout().flush().unwrap();
        let mut choice: String = String::new();
        std::io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice: u8 = choice.trim().parse().expect("Enter a valid input");

        match choice {
            1 => {
                tasks.add_new_task();
                tasks.save_to_file("tasks.json").expect("failed to save new task.");
            },
            2 => {
                tasks.update_task();
                tasks.save_to_file("tasks.json").expect("Failed to save update task.");
            },
            3 => {
                if let Some(task) = tasks.delete() {
                    // Use the deleted task here
                    println!("Deleted task: {}\nDescription: {}", task.title, task.description);
                    tasks.save_to_file("tasks.json").expect("Failed to save delete task.");
                } else {
                    println!("No task deleted.");
                }
            },
            4 => tasks.display_all(),
            5 => break,
            _ => {
                println!("❌ Invalid input. Try again");
                continue; // Jump back to the start of the loop
            }, 
        }
    }
}