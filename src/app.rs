// use serde::{Deserialize, Serialize};

/// Used to jump between task subsections while editng
pub enum CurrentlyEditing {
    Task,
    DueDate,
    Status,
    Priority,
}

/// This enum is used to jump between screens.
pub enum CurrentScreen {
    New,     // screen to add a new task
    Main,    // screen to display all the tasks
    Task,    // screen to display a specific task
    Editing, // screen to modify a specific task
    Exiting, // screen to confirm exit
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

    pub fn change_screen(&mut self, screen: CurrentScreen) {
        self.current_screen = screen;
    }

    pub fn change_editing(&mut self, edit: Option<CurrentlyEditing>) {
        self.currently_editing = edit;
    }

    // pub fn next_edit(&mut self,)
}
