use serde::{Deserialize, Serialize};
// use std::fs::File;
// use std::io::{self, Read, Write};

// #[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    NotStarted,
    Ongoing,
    Completed, // Just like my to-do list: never started, always ongoing, never completed!
}

// #[derive(Debug, Serialize, Deserialize)]
pub enum Priority {
    Urgent,
    Important,
    Normal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub task: String,
    // pub status: Status,
    // pub priority: Priority,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskManager {
    pub input_task: String,
    pub tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            input_task: String::new(),
            tasks: Vec::new(),
        }
    }
    pub fn insert_tasks(&mut self, tasks: Vec<Task>) {
        self.tasks = tasks;
    }
}
