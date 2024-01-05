mod task;
mod task_manager;
use crate::task_manager::TaskManager;
use colored::Colorize;
use std::io::Write;

fn main() {
    let mut tasks: TaskManager = TaskManager::load_from_file("tasks.json")
        .unwrap_or_else(|_| TaskManager { tasks: Vec::new() });

    println!("Hello User! How can I help you?");
    loop {
        print!("\n******************************\n✅ {0}\n➕ {1}\n⛔ {2}\n🫙 {3}\n📤 {4}\nChoose an option => ", "1. Add Task".bright_yellow(), "2. Update Task".bright_yellow(), "3. Delete Task".bright_yellow(), "4. Display list".bright_yellow(), "5. Exit".red());

        std::io::stdout().flush().unwrap();
        let mut choice: String = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");
        let choice: u8 = choice.trim().parse().expect("Enter a valid input");

        match choice {
            1 => {
                tasks.add_new_task();
                tasks.save_to_file("tasks.json")
                    .expect("failed to save new task.");
            }
            2 => {
                tasks.update_task();
                tasks.save_to_file("tasks.json")
                    .expect("Failed to save update task.");
            }
            3 => {
                if let Some(task) = tasks.delete() {
                    println!("Deleted task: {}", task.task);
                    tasks.save_to_file("tasks.json")
                        .expect("Failed to save delete task.");
                } else {
                    println!("No task deleted.");
                }
            }
            4 => tasks.display_all(),
            5 => break,
            _ => {
                println!("❌ Invalid input. Try again");
                continue;
            }
        }
    }
}
