use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Style, Stylize},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

#[allow(dead_code)]
/// This function renders the introduction page on the terminal.
pub fn introduction_frame(frame: &mut Frame) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(6), // this is where the output will display
            Constraint::Max(3), // this is where the input will display
        ])
        .split(frame.size());

    let top_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Ratio(2, 5), // this is empty space
            Constraint::Ratio(3, 5), // This will show Task Manager
        ])
        .split(main_layout[0]);

    let empty_space = Block::new().borders(Borders::TOP | Borders::LEFT | Borders::RIGHT);
    frame.render_widget(empty_space, top_layout[0]);

    let introduction = Paragraph::new(Text::styled(
        "Task Manager TUI\nby A. Shaikh",
        Style::default().bold().fg(ratatui::style::Color::Green),
    ))
    .block(Block::default().borders(Borders::BOTTOM | Borders::LEFT | Borders::RIGHT))
    .alignment(Alignment::Center);

    frame.render_widget(introduction, top_layout[1]);

    let controls = Paragraph::new(Text::styled(
        "Press [enter] to continue",
        Style::default().fg(ratatui::style::Color::LightCyan),
    ))
    .block(Block::default().borders(Borders::ALL))
    .alignment(Alignment::Center)
    .rapid_blink();

    frame.render_widget(controls, main_layout[1]);
}
