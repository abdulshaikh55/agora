use std::fs;
enum Priority {
    Low, Medium, High
}
enum Status {
    NotStarted, Ongoing, Completed
}
struct Task {
    title: String,
    description: String,
    priority: Priority,
    due_date: String,
    status: Status
}

pub mod TaskHandling{
    pub fn add_task() {

    }

    pub fn update_task() {

    }

    pub fn delete_task() {

    }

}