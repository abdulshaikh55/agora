pub enum CurrentScreen {
    Main,    // screen to display all the tasks
    Task,    // screen to display a specific task
    Editing, // screen to modify a specific task
    Exiting, // screen to confirm exit
}

pub enum CurrentlyEditing {
    Task, DueDate, Status, Priority, 
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
}

impl App {
    pub fn new() -> Self {
        App { 
            current_screen: CurrentScreen::Main, 
            currently_editing: None,        }
    }
}