use std::ops::Deref;

use ratatui::widgets::ListState;

use crate::task_management::Task;
pub struct StatefulList {
    pub state: ListState,
    pub tasks: Vec<Task>,
}

impl StatefulList {
    /// This function creates a new StatefulList with default state of the list.
    pub fn new(tasks: &Vec<Task>) -> Self {
        Self {
            state: ListState::default(),
            tasks: Into::<Vec<_>>::into(tasks.deref()),
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
            }
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
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    /// Unselect whatever the state pointing to.
    pub fn unselect(&mut self) {
        self.state.select(None);
    }

    // pub fn select(&mut self) -> Option<usize> {
    //     self.state.selected()
    // }

    /// This function extracts only the tasks in the task manager and appends it
    pub fn extract_task_string_only(&mut self) -> Vec<String> {
        self.tasks
            .iter()
            .map(|task| format!("{}", &task.task))
            .collect()
    }

    pub fn extract_specific_task_string_only(&mut self, idx: usize) -> String {
        let task = self.tasks.get(idx).unwrap();
        task.task.clone()
    }

    // fn unchange_selected_string(&mut self) {}
}

#[cfg(test)]

mod controls_tests {
    use super::*;

    #[test]
    fn test_select_down() {
        let tasks: Vec<Task> = vec![
            Task {
                task: "Eat".to_string(),
            },
            Task {
                task: "Code".to_string(),
            },
            Task {
                task: "Sleep".to_string(),
            },
            Task {
                task: "Repeat".to_string(),
            },
        ];

        let mut list_with_state = StatefulList::new(&tasks);
        list_with_state.next();

        assert_eq!(
            Some(0),
            list_with_state.state.selected(),
            "ListState not initialized"
        );
        list_with_state.next();

        assert_eq!(
            Some(1),
            list_with_state.state.selected(),
            "ListState not incremented"
        );
    }

    #[test]
    fn test_select_up() {
        let tasks: Vec<Task> = vec![
            Task {
                task: "Eat".to_string(),
            },
            Task {
                task: "Code".to_string(),
            },
            Task {
                task: "Sleep".to_string(),
            },
            Task {
                task: "Repeat".to_string(),
            },
        ];

        let mut list_with_state = StatefulList::new(&tasks);
        list_with_state.previous();

        assert_eq!(
            Some(0),
            list_with_state.state.selected(),
            "ListState not initialized"
        );
        list_with_state.previous();

        assert_eq!(
            Some(list_with_state.tasks.len() - 1),
            list_with_state.state.selected(),
            "ListState not decremented"
        );
    }

    #[test]
    fn test_unselect() {
        let tasks: Vec<Task> = vec![
            Task {
                task: "Eat".to_string(),
            },
            Task {
                task: "Code".to_string(),
            },
            Task {
                task: "Sleep".to_string(),
            },
            Task {
                task: "Repeat".to_string(),
            },
        ];

        let mut list_with_state = StatefulList::new(&tasks);
        list_with_state.previous();
        list_with_state.unselect();
        assert_eq!(
            None,
            list_with_state.state.selected(),
            "ListState not unselected"
        );
        list_with_state.previous();
    }
}
