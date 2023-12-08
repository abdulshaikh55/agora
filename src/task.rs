use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    NotStarted,
    Ongoing,
    Completed, // Just like my to-do list: never started, always ongoing, never completed!
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Priority {
    Urgent,
    Important,
    Normal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub task: String,
    pub due_date: String,
    pub status: Status,
    pub priority: Priority,
}
