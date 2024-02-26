use ratatui::widgets::ListState;

pub struct StatefulList {
    pub state: ListState,
    pub tasks: Vec<String>,
}

impl StatefulList {
    /// This function creates a new StatefulList with default state of the list.
    pub fn new(tasks: Vec<String>) -> Self {
        Self {
            state: ListState::default(),
            tasks,
        }
    }

    /// When called, this function initially sets the state of the list to 0. <br>
    /// If already pointing to Some item in the list, it will point to the next item.
    /// # Example 
    /// if state = 3:  
    /// after next is called,  
    /// state = 4.  
    /// If the state is pointing to the last item in the list, after next is called, it will point to the 0th item.
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.tasks.len() - 1 {
                    0
                } else {
                    i + 1
                }
            },
            None => 0,
        };
        self.state.select(Some(i));
    }

    /// When called, this function intitally sets the state of the list to 0.  
    /// If already pointing to Some item in the list, it will point to the previous item.  
    /// # Example
    /// if state = 3:  
    /// after previous is called,  
    /// state = 2.  
    /// If state is pointing to the 0th item in the list, after previous is called, it will point to the last item.
    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.tasks.len() - 1
                } else {
                    i - 1
                }
            },
            None => 0,
        };
        self.state.select(Some(i));
    }

    /// Unselect whatever the state pointing to.
    pub fn unselect(&mut self) {
        self.state.select(None);
    }

    pub fn select(&mut self) {
        if self.state.selected() == None {
            todo!("return the task index, so that it can be displayed on CurrentScreen::Task")
        } else {
            todo!()
        }
    }
}

#[cfg(test)]

mod controls_tests {
    use super::*;

    #[test]
    fn test_select_down() {
        let tasks = vec!["test task 1".to_string(), "test task 2".to_string(), "test task 3".to_string()];

        let mut list_with_state = StatefulList::new(tasks);
        list_with_state.next();

        assert_eq!(Some(0), list_with_state.state.selected(), "ListState not initialized");
        list_with_state.next();

        assert_eq!(Some(1), list_with_state.state.selected(), "ListState not incremented");
    }

    #[test]
    fn test_select_up() {
        let tasks = vec!["test task 1".to_string(), "test task 2".to_string(), "test task 3".to_string()];

        let mut list_with_state = StatefulList::new(tasks);
        list_with_state.previous();

        assert_eq!(Some(0), list_with_state.state.selected(), "ListState not initialized");
        list_with_state.previous();

        assert_eq!(Some(list_with_state.tasks.len() - 1), list_with_state.state.selected(), "ListState not decremented");
    }

    #[test]
    fn test_unselect() {
        let tasks = vec!["test task 1".to_string(), "test task 2".to_string(), "test task 3".to_string()];

        let mut list_with_state = StatefulList::new(tasks);
        list_with_state.previous();
        list_with_state.unselect();
        assert_eq!(None, list_with_state.state.selected(), "ListState not unselected");
        list_with_state.previous();

    }
}