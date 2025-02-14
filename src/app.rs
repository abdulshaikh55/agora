// use serde::{Deserialize, Serialize};

/// Used to jump between task subsections while editng
pub enum CurrentlyEditing {
    Task,
    Status,
    Priority,
}

/// This enum is used to jump between screens.
pub enum CurrentScreen {
    New,
    Main,
    Delete,
    Task,
    Editing,
    Exiting,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub currently_editing: CurrentlyEditing,
}

impl App {
    pub fn new() -> Self {
        App {
            current_screen: CurrentScreen::Main,
            currently_editing: CurrentlyEditing::Task,
        }
    }

    pub fn change_screen(&mut self, screen: CurrentScreen) {
        self.current_screen = screen;
    }

    pub fn toggle_task_priority(&mut self) {
        match self.currently_editing {
            CurrentlyEditing::Task => self.currently_editing = CurrentlyEditing::Priority,
            CurrentlyEditing::Priority => self.currently_editing = CurrentlyEditing::Task,
            CurrentlyEditing::Status => self.currently_editing = CurrentlyEditing::Task,
        }
    }

    pub fn toggle_priority_status(&mut self) {
        match self.currently_editing {
            CurrentlyEditing::Status => self.currently_editing = CurrentlyEditing::Priority,
            CurrentlyEditing::Priority => self.currently_editing = CurrentlyEditing::Status,
            _ => (),
        }
    }
}
