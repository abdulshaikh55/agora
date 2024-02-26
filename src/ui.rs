use ratatui::{
    layout::{Constraint, Direction, Layout, Rect}, 
    style::{Modifier, Style, Stylize}, 
    symbols, 
    text::{Span, Text}, 
    widgets::{Block, Borders, /* Clear,*/ List, Paragraph, Wrap}, 
    Frame
};

use crate::{app::CurrentScreen, controls::StatefulList};
use crate::app::App;
pub fn ui(frame: &mut Frame, list_with_state: &mut StatefulList, app: &App) {

    let layout = Layout::new(
        Direction::Vertical, 
        [
            Constraint::Length(3),
            Constraint::Min(2),
            Constraint::Length(3),
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
        .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
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

    // let navigation: Span<'_> = {
    //     match app.current_screen {

    //     }
    // }

    let footer_chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
        Constraint::Percentage(30),
        Constraint::Percentage(70),
    ])
    .split(layout[2]);


    let navigation = Paragraph::new(Span::styled(" Main Menu", 
    Style::default().fg(ratatui::style::Color::Gray)))
        .block(Block::default().borders(Borders::ALL));



    let control_panel= Paragraph::new(Span::styled("[⬆] / [⬇] to move to your task, [➡️] to select [⬅] to unselect, [q] to quit",
    Style::default().fg(ratatui::style::Color::Green)))
        .block(Block::default().borders(Borders::ALL));

    
    
    frame.render_widget(navigation, footer_chunks[0]);
    frame.render_widget(control_panel, footer_chunks[1]);


    if let CurrentScreen::Exiting = app.current_screen {
        // frame.render_widget(Clear, frame.size()); // this clears the entire screen and anything already drawn
        let popup_block = Block::default()
            .borders(Borders::ALL)
            .border_set(symbols::border::DOUBLE)
            .style(Style::default().fg(ratatui::style::Color::DarkGray));

        let exit_text = Text::styled("Do you want to exit Task Manager? [y]/[n]", 
            Style::default().fg(ratatui::style::Color::Red));

        let exit_paragraph = Paragraph::new(exit_text)
            .block(popup_block)
            .wrap(Wrap {trim: false});

        let area = centered_rect(60, 25, frame.size());
        frame.render_widget(exit_paragraph, area);
    }

    
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece itno three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}