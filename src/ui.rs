use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout}, style::{Modifier, Style, Stylize}, symbols, text::Text, widgets::{Block, Borders, List, ListState, Paragraph}, Frame
};


pub fn ui(frame: &mut Frame, list_with_state: &mut StatefulList) {

   
    let layout = Layout::new(
        Direction::Vertical, 
        [
            Constraint::Length(3),
            Constraint::Min(2)
        ]
    ).split(frame.size());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .border_set(symbols::border::ROUNDED);

    let title_style = Style::default().fg(ratatui::style::Color::Green).bold();

    let title = Paragraph::new(Text::styled("Task Manager", title_style)).block(title_block).alignment(ratatui::layout::Alignment::Center);

    frame.render_widget(title, layout[0]); 

    let list_block =Block::default()
        .title("List")
        .borders(Borders::ALL)
        .white();

    let list_style = Style::default().fg(ratatui::style::Color::Cyan);

    let list = List::new(list_with_state.tasks.clone())
        .block(list_block)
        .style(list_style)
        .highlight_style(Style::default()
            .fg(ratatui::style::Color::White)
            .add_modifier(Modifier::BOLD)
        )
        .highlight_symbol("* ");

    frame.render_stateful_widget(list, layout[1], &mut list_with_state.state);
    
}

pub fn introduction(frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // This will show Task Manager
            Constraint::Max(3),
        ]).split(frame.size());
    
    let project_name = Paragraph::new(Text::styled("Welcome to Task Manager\nBy Abdulfaiz Shaikh", 
        Style::default().bold().fg(ratatui::style::Color::LightGreen)
        )
    )
    .block(Block::default().borders(Borders::ALL).title_alignment(Alignment::Center))
    .alignment(Alignment::Center);

    frame.render_widget(project_name, layout[0]);

    let writer_name = Paragraph::new(Text::styled("Press [enter] to continue", 
        Style::default().fg(ratatui::style::Color::LightCyan)
    )).block(Block::default().borders(Borders::ALL))
    .alignment(Alignment::Center)
    .rapid_blink();

    frame.render_widget(writer_name, layout[1]);

}


pub struct StatefulList {
    state: ListState,
    tasks: Vec<String>,
}

impl StatefulList {
    /// This function creates a new StatefulList with default state of the list.
    pub fn new(tasks: Vec<String>) -> Self {
        StatefulList {
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
}