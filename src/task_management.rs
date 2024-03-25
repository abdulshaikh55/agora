use ratatui::widgets::ListState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Status {
    NotStarted,
    Ongoing,
    Completed,
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
    pub fn new(tasks: Vec<Task>) -> Self {
        Self {
            input_task_string: String::new(),
            input_priority: Priority::Important,
            input_status: Status::NotStarted,
            tasks,
        }
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

    pub fn save_instance_task(&mut self) {
        self.tasks.push(Task {
            task: self.input_task_string.clone(),
            priority: self.input_priority.clone(),
            status: self.input_status.clone(),
        });

        self.input_task_string = String::new();
        self.input_priority = Priority::Important;
        self.input_status = Status::NotStarted;
    }
}

use std::ops::Deref;

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
    use crate::task_management::Priority;
    use crate::task_management::Status;

    #[test]
    fn test_select_down() {
        let tasks: Vec<Task> = vec![
            Task {
                task: "Eat".to_string(),
                priority: Priority::Important,
                status: Status::NotStarted,
            },
            Task {
                task: "Code".to_string(),
                priority: Priority::Important,
                status: Status::NotStarted,
            },
            Task {
                task: "Sleep".to_string(),
                priority: Priority::Important,
                status: Status::NotStarted,
            },
            Task {
                task: "Repeat".to_string(),
                priority: Priority::Important,
                status: Status::NotStarted,
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
                priority: Priority::Important,
                status: Status::NotStarted,
            },
            Task {
                task: "Code".to_string(),
                priority: Priority::Important,
                status: Status::NotStarted,
            },
            Task {
                task: "Sleep".to_string(),
                priority: Priority::Important,
                status: Status::NotStarted,
            },
            Task {
                task: "Repeat".to_string(),
                priority: Priority::Important,
                status: Status::NotStarted,
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
                priority: Priority::Important,
                status: Status::NotStarted,
            },
            Task {
                task: "Code".to_string(),
                priority: Priority::Important,
                status: Status::NotStarted,
            },
            Task {
                task: "Sleep".to_string(),
                priority: Priority::Important,
                status: Status::NotStarted,
            },
            Task {
                task: "Repeat".to_string(),
                priority: Priority::Important,
                status: Status::NotStarted,
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
