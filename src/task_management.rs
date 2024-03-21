use serde::{Deserialize, Serialize};
// use std::fs::File;
// use std::io::{self, Read, Write};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Status {
    NotStarted,
    Ongoing,
    Completed, // Just like my to-do list: never started, always ongoing, never completed!
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Priority {
    Urgent,
    Important,
    Normal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub task: String,
    pub priority: Priority,
    pub status: Status,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskManager {
    pub input_task_string: String,
    pub input_priority: Priority,
    pub input_status: Status,
    pub tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            input_task_string: String::new(),
            input_priority: Priority::Important,
            input_status: Status::NotStarted,
            tasks: Vec::new(),
        }
    }
    pub fn insert_tasks(&mut self, tasks: Vec<Task>) {
        self.tasks = tasks;
    }

    pub fn switch_priority_value(&mut self) {
        match self.input_priority {
            Priority::Urgent => self.input_priority = Priority::Normal,
            Priority::Important => self.input_priority = Priority::Urgent,
            Priority::Normal => self.input_priority = Priority::Important,
        }
    }

    pub fn switch_status_value(&mut self) {
        match self.input_status {
            Status::NotStarted => self.input_status = Status::Ongoing,
            Status::Ongoing => self.input_status = Status::Completed,
            Status::Completed => self.input_status = Status::NotStarted,
        }
    }
}
