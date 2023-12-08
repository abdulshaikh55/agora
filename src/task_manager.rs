use crate::task::{Priority, Status, Task};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Read, Write};

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskManager {
    pub tasks: Vec<Task>,
}

impl TaskManager {
    pub fn add_new_task(&mut self) {
        let mut task = String::new();
        let mut due_date = String::new();

        print!("Enter task: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut task)
            .expect("Failed to read input");
        let task: String = task.trim().to_string();

        print!("Enter due date(DD-MM-YYYY): ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut due_date)
            .expect("Failed to read input");
        let due_date = due_date.trim().to_string();

        let prior = self.get_priority();

        let new_task = Task {
            task,
            due_date,
            status: Status::NotStarted,
            priority: prior,
        };

        self.tasks.push(new_task);
        println!("{}", "\nTask successfully added!\n".green());
    }

    fn get_priority(&self) -> Priority {
        println!("Choose task priority: ");
        println!("u - Urgent\ni - Important\nn - Normal");
        let mut priority = String::new();
        io::stdin()
            .read_line(&mut priority)
            .expect("Failed to read input");
        let priority: char = priority.trim().parse().expect("Enter a valid choice");
        match priority {
            'u' => Priority::Urgent,
            'i' => Priority::Important,
            'n' => Priority::Normal,
            _ => panic!("Invalid choice"),
        }
    }

    pub fn display_all(&self) {
        for (idx, task) in self.tasks.iter().enumerate() {
            println!(
                "{}",
                "\n**************************************************".yellow()
            );
            println!("Priority: {:?}", task.priority);
            println!("{} -> {}", (idx + 1), task.task);
            println!("Due Date: {}", task.due_date);
            println!("Status: {:?}", task.status);
            println!(
                "{}",
                "**************************************************".yellow()
            );
        }
    }

    pub fn update_task(&mut self) {
        loop {
            self.display_all();

            print!("Choose the task you want to update (Enter 0 to exit): ");
            io::stdout().flush().unwrap();
            let mut choice: String = String::new();
            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read input");
            let choice: usize = choice.trim().parse().expect("Enter a valid input");

            if choice > self.tasks.len() {
                println!("The task at that index does not exist. Try again.");
                continue;
            }
            if choice == 0 {
                break;
            }

            self.modify_task(choice);
        }
    }

    fn modify_task(&mut self, choice: usize) {
        match self.tasks.get_mut(choice - 1) {
            Some(task) => {
                println!("Choose the aspect you want to update");
                println!("1. Task");
                println!("2. Due Date");
                println!("3. Priority");
                println!("4. Status");
                print!("{}", "5. Exit\n=> ".red());
                io::stdout().flush().unwrap();
                let mut modification_choice = String::new();
                io::stdin()
                    .read_line(&mut modification_choice)
                    .expect("Failed to read input");
                let modify_choice: u8 =
                    modification_choice.trim().parse().expect("Enter a valid input");

                match modify_choice {
                    1 => {
                        let mut task_str = String::new();
                        print!("Enter new task: ");
                        io::stdout().flush().unwrap();
                        io::stdin()
                            .read_line(&mut task_str)
                            .expect("Failed to read the task");
                        task.task = task_str.trim().to_string();
                    }
                    2 => {
                        let mut due_date = String::new();
                        print!("Enter new due date(DD/MM/YY): ");
                        io::stdout().flush().unwrap();
                        io::stdin()
                            .read_line(&mut due_date)
                            .expect("Failed to read due_date");
                        task.due_date = due_date.trim().to_string();
                    }
                    3 => {
                        println!("Choose task priority: ");
                        println!("u - Urgent\ni - Important\nn - Normal");
                        let mut priority = String::new();
                        io::stdin()
                            .read_line(&mut priority)
                            .expect("Failed to read input");
                        let priority: char = priority.trim().parse().expect("Enter a valid choice");
                        let prior = match priority {
                            'u' => Priority::Urgent,
                            'i' => Priority::Important,
                            'n' => Priority::Normal,
                            _ => panic!("Invalid choice"),
                        };
                        task.priority = prior;
                    }
                    4 => {
                        if let Some(task) = self.tasks.get_mut(choice - 1) {
                            let mut status = String::new();
        print!("Enter new status: \n1. Not started\n2. Ongoing\n3. Completed\n => ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut status)
            .expect("Failed to read status");
        let status = match status.trim().parse() {
            Ok(1) => Status::NotStarted,
            Ok(2) => Status::Ongoing,
            Ok(3) => Status::Completed,
            _ => {
                println!("❌ Invalid status ❌");
                Status::NotStarted
            }
        };
        task.status = status;
                        } else {
                            println!("🚫 There is no Task with that index 🚫");
                        }
                    }
                    5 => return,
                    _ => {
                        println!("❌ Invalid choice ❌");
                    }
                }
            }
            None => println!("🚫 There is no Task with that index 🚫"),
        }
    }

    pub fn delete(&mut self) -> Option<Task> {
        loop {
            let mut choice: String = String::new();
            println!(
                "1. Delete last task\n2. Delete all tasks\n3. Delete specific task\n4. Cancel"
            );
            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read input");
            let choice: u8 = choice.trim().parse().expect("Enter a valid input");
            match choice {
                1 => {
                    if let Some(task) = self.tasks.pop() {
                        return Some(task);
                    }
                }
                2 => {
                    self.tasks.clear();
                    println!("All tasks deleted.");
                }
                3 => {
                    let mut idx: String = String::new();
                    print!("Enter the index of the task: ");
                    io::stdout().flush().unwrap();
                    io::stdin()
                        .read_line(&mut idx)
                        .expect("Failed to read input");
                    let idx: usize = idx.trim().parse().expect("Enter a valid input");
                    let task = self.tasks.remove(idx - 1);
                    return Some(task);
                }
                4 => break,
                _ => {
                    println!("Please select a valid choice.");
                    continue;
                }
            }
        }
        None
    }

    pub fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let data = serde_json::to_string(&self)?;
        let mut file = File::create(filename)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    pub fn load_from_file(filename: &str) -> io::Result<TaskManager> {
        let mut file = File::open(filename)?;
        let mut file_content = String::new();
        file.read_to_string(&mut file_content)?;
        let tasks: TaskManager = serde_json::from_str(&file_content)?;
        Ok(tasks)
    }
}
