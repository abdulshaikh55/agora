extern crate tui;
use tui::{Terminal, terminal};
use tui::backend::CrosstermBackend;
use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Layout, Constraint, Direction};
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

fn main() {
    // Create a TUI backend for the Terminal
    let backend = CrosstermBackend::new(io::stdout());

    // Create a TUI terminal using the backend
    let mut terminal = terminal::new(backend).unwrap();
}