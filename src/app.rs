// use serde::{Deserialize, Serialize};

pub enum CurrentScreen {
    Main,    // screen to display all the tasks
    Task,    // screen to display a specific task
    Editing, // screen to modify a specific task
    Exiting, // screen to confirm exit
}

pub enum CurrentlyEditing {
    Task,
    DueDate,
    Status,
    Priority,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
}

impl App {
    pub fn new() -> Self {
        App {
            current_screen: CurrentScreen::Main,
            currently_editing: None,
        }
    }
}

mod task {

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

    // #[derive(Debug, Serialize, Deserialize)]
    pub struct Task {
        pub task: String,
        // pub due_date: String,
        // pub status: Status,
        // pub priority: Priority,
    }
}
