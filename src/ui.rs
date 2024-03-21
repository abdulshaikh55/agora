use std::rc::Rc;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols,
    text::{Line, Text},
    widgets::{Block, Borders, List, Paragraph, Wrap},
    Frame,
};

use crate::{app::App, task_management::Status};
use crate::{app::CurrentScreen, controls::StatefulList};
use crate::{
    app::CurrentlyEditing,
    task_management::{Priority, TaskManager},
};

pub fn ui(
    frame: &mut Frame,
    list_with_state: &mut StatefulList,
    app: &App,
    task_manager: &TaskManager,
) {
    let layout: Rc<[Rect]> = create_main_layout(frame.size());

    render_title(frame, layout[0]);

    render_list(frame, layout[1], list_with_state);

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(layout[2]);

    render_footer(frame, footer_chunks, app);

    if let CurrentScreen::New = app.current_screen {
        let area = centered_rect(50, 40, frame.size());
        let popup_new_layout = Layout::new(
            Direction::Vertical,
            [Constraint::Length(4), Constraint::Length(4)],
        )
        .split(area);

        let second_section = Layout::new(
            Direction::Horizontal,
            [Constraint::Percentage(50), Constraint::Percentage(50)],
        )
        .split(popup_new_layout[1]);

        render_task_input_box(
            frame,
            &task_manager,
            popup_new_layout[0],
            &app.currently_editing,
        );
        render_priority_input_box(
            frame,
            &task_manager,
            second_section[0],
            &app.currently_editing,
        );
        render_status_input_box(
            frame,
            &task_manager,
            second_section[1],
            &app.currently_editing,
        );

        // When you enter a Task section, this popup will appear
        if let CurrentScreen::Task = app.current_screen {
            // frame.render_widget(Clear, frame.size()); // this clears the entire screen and anything already drawn
            let popup_task_block = Block::default()
                .borders(Borders::ALL)
                .border_set(symbols::border::DOUBLE)
                .style(Style::default().bg(ratatui::style::Color::DarkGray));

            let task_string: String;
            if list_with_state.state.selected() == None {
                task_string = "No task Selected".to_string();
            } else {
                let idx = list_with_state.state.selected().unwrap();
                // we are using tasks : Vec<String>
                task_string = format!(
                    "  {}",
                    list_with_state.extract_specific_task_string_only(idx)
                );
                // create variable of selected task.
            }

            let styled_task = Text::styled(
                task_string,
                Style::default().fg(ratatui::style::Color::Green).bold(),
            );
            let display_task = Paragraph::new(styled_task)
                .block(popup_task_block)
                .wrap(Wrap { trim: false });
            let area = centered_rect(50, 50, frame.size());

            frame.render_widget(display_task, area);
        }

        if let CurrentScreen::Exiting = app.current_screen {
            // frame.render_widget(Clear, frame.size()); // this clears the entire screen and anything already drawn
            let popup_block = Block::default()
                .borders(Borders::ALL)
                .border_set(symbols::border::DOUBLE)
                .style(Style::default().fg(ratatui::style::Color::DarkGray));

            let exit_text = Text::styled(
                " Do you want to exit Task Manager?",
                Style::default().fg(ratatui::style::Color::Red),
            );

            let exit_paragraph = Paragraph::new(exit_text)
                .block(popup_block)
                .wrap(Wrap { trim: false });

            let area = centered_rect(60, 25, frame.size());
            frame.render_widget(exit_paragraph, area);
        }
    }
}

/// This function creates the main layout of three blocks vertically.
fn create_main_layout(size: Rect) -> Rc<[Rect]> {
    Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(3),
            Constraint::Min(2),
            Constraint::Length(3),
        ],
    )
    .split(size)
}

fn render_title(frame: &mut Frame, area: Rect) {
    let title_block = Block::default()
        .borders(Borders::ALL)
        .border_set(symbols::border::ROUNDED);
    let title_style = Style::default().fg(ratatui::style::Color::Green).bold();
    let title = Paragraph::new(Text::styled("Task Manager", title_style))
        .block(title_block)
        .alignment(ratatui::layout::Alignment::Center);

    frame.render_widget(title, area);
}

fn render_list(frame: &mut Frame, area: Rect, list_with_state: &mut StatefulList) {
    let list_block = Block::default()
        .title("List")
        .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
        .white();
    let list_style = Style::default().fg(ratatui::style::Color::Cyan);
    let list = List::new(list_with_state.extract_task_string_only())
        .block(list_block)
        .style(list_style)
        .highlight_style(
            Style::default()
                .fg(ratatui::style::Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("[*]");

    frame.render_stateful_widget(list, area, &mut list_with_state.state);
}

fn render_footer(frame: &mut Frame, area: Rc<[Rect]>, app: &App) {
    let navigation = match app.current_screen {
        CurrentScreen::Main => Line::styled(
            " Main Menu",
            Style::default().fg(ratatui::style::Color::Gray),
        ),
        CurrentScreen::Editing => Line::styled(
            " Editing",
            Style::default().fg(ratatui::style::Color::Green),
        ),
        CurrentScreen::Task => Line::styled(
            " Task View",
            Style::default().fg(ratatui::style::Color::Yellow),
        ),
        CurrentScreen::Exiting => {
            Line::styled(" Exiting", Style::default().fg(ratatui::style::Color::Red))
        }
        CurrentScreen::New => Line::styled(
            " New Task",
            Style::default().fg(ratatui::style::Color::Blue),
        ),
    };
    let navigation = Paragraph::new(navigation).block(Block::default().borders(Borders::ALL));
    frame.render_widget(navigation, area[0]);

    let control_panel = match app.current_screen {
        CurrentScreen::Main => Line::styled(
            " [➡] / [⬅] select / unselect, [Enter] for new, [Esc] to exit",
            Style::default().fg(ratatui::style::Color::Green),
        ),
        CurrentScreen::Editing => Line::styled(
            " [⬆] / [⬇] to move, [➡] to select [⬅] to unselect",
            Style::default().fg(ratatui::style::Color::Green),
        ),
        CurrentScreen::Task => Line::styled(
            " [⬆] / [⬇] to move, [➡] to Edit [⬅] to Main Menu",
            Style::default().fg(ratatui::style::Color::Green),
        ),
        CurrentScreen::Exiting => Line::styled(
            " [y] for yes, [n] for no",
            Style::default().fg(ratatui::style::Color::Green),
        ),
        CurrentScreen::New => Line::styled(
            " [Esc] to go back, [Enter] to continue",
            Style::default().fg(ratatui::style::Color::Green),
        ),
    };
    let control_panel = Paragraph::new(control_panel).block(Block::default().borders(Borders::ALL));
    frame.render_widget(control_panel, area[1]);
}

fn render_task_input_box(
    frame: &mut Frame,
    task_manager: &TaskManager,
    area: Rect,
    active: &CurrentlyEditing,
) {
    let new_task_block = Block::default()
        .title("Task")
        .borders(Borders::ALL)
        .border_set(symbols::border::DOUBLE);

    let task_input_display = Paragraph::new(task_manager.input_task_string.clone())
        .block(new_task_block)
        .fg(match active {
            CurrentlyEditing::Task => Color::Black,
            _ => Color::White,
        })
        .bg(match active {
            CurrentlyEditing::Task => Color::Yellow,
            _ => Color::Black,
        });

    frame.render_widget(task_input_display, area);
}

fn render_priority_input_box(
    frame: &mut Frame,
    task_manager: &TaskManager,
    area: Rect,
    active: &CurrentlyEditing,
) {
    // Setting up the blocks for |priority | status|
    let new_priority_block = Block::default()
        .title("Priority")
        .borders(Borders::ALL)
        .border_set(symbols::border::DOUBLE);

    frame.render_widget(new_priority_block.clone(), area);

    let display_priority = match task_manager.input_priority {
        Priority::Urgent => Paragraph::new("Urgent")
            .alignment(ratatui::layout::Alignment::Center)
            .fg(match active {
                CurrentlyEditing::Priority => Color::Black,
                _ => Color::White,
            })
            .bg(match active {
                CurrentlyEditing::Priority => Color::Red,
                _ => Color::Black,
            })
            .block(Block::default()),

        Priority::Important => Paragraph::new("Important")
            .alignment(ratatui::layout::Alignment::Center)
            .fg(match active {
                CurrentlyEditing::Priority => Color::Black,
                _ => Color::White,
            })
            .bg(match active {
                CurrentlyEditing::Priority => Color::Blue,
                _ => Color::Black,
            })
            .block(Block::default()),

        Priority::Normal => Paragraph::new("Normal")
            .alignment(ratatui::layout::Alignment::Center)
            .fg(match active {
                CurrentlyEditing::Priority => Color::Black,
                _ => Color::White,
            })
            .bg(match active {
                CurrentlyEditing::Priority => Color::Green,
                _ => Color::Black,
            })
            .block(Block::default()),
    }
    .block(new_priority_block);

    frame.render_widget(display_priority, area);
}

fn render_status_input_box(
    frame: &mut Frame,
    task_manager: &TaskManager,
    area: Rect,
    active: &CurrentlyEditing,
) {
    let new_status_block = Block::default()
        .title("Status")
        .borders(Borders::ALL)
        .border_set(symbols::border::DOUBLE);

    frame.render_widget(new_status_block.clone(), area);

    let display_status = match task_manager.input_status {
        Status::NotStarted => Paragraph::new("Not Started")
            .alignment(ratatui::layout::Alignment::Center)
            .fg(match active {
                CurrentlyEditing::Status => Color::Black,
                _ => Color::White,
            })
            .bg(match active {
                CurrentlyEditing::Status => Color::Red,
                _ => Color::Black,
            })
            .block(Block::default()),

        Status::Ongoing => Paragraph::new("Ongoing")
            .alignment(ratatui::layout::Alignment::Center)
            .fg(match active {
                CurrentlyEditing::Status => Color::Black,
                _ => Color::White,
            })
            .bg(match active {
                CurrentlyEditing::Status => Color::Blue,
                _ => Color::Black,
            })
            .block(Block::default()),

        Status::Completed => Paragraph::new("Complete")
            .alignment(ratatui::layout::Alignment::Center)
            .fg(match active {
                CurrentlyEditing::Status => Color::Black,
                _ => Color::White,
            })
            .bg(match active {
                CurrentlyEditing::Status => Color::Green,
                _ => Color::Black,
            })
            .block(Block::default()),
    }
    .block(new_status_block);

    frame.render_widget(display_status, area);
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

#[cfg(test)]
mod ui_test {
    use super::*;

    #[test]
    fn test_centered_rect() {
        let rect = Rect {
            x: 0,
            y: 0,
            width: 200,
            height: 150,
        };
        let expected_rect = Rect {
            x: 74,
            y: 18,
            width: 50,
            height: 113,
        };
        let actual_rect = centered_rect(25, 75, rect);
        assert_eq!(actual_rect, expected_rect);
    }

    #[test]
    fn test_create_main_layout() {
        let screen_size = Rect {
            x: 0,
            y: 0,
            width: 80,
            height: 25,
        };

        let layout: Rc<[Rect]> = create_main_layout(screen_size);

        let expected_sizes: Rc<[Rect]> = [
            // Title: 3 units of height (assuming full width)
            Rect {
                x: 0,
                y: 0,
                width: 80,
                height: 3,
            },
            // List: Min height of 2 (assuming full width)
            Rect {
                x: 0,
                y: 3,
                width: 80,
                height: 19,
            },
            // Footer: 3 units of height (assuming full width)
            Rect {
                x: 0,
                y: 22,
                width: 80,
                height: 3,
            },
        ]
        .into();

        for (i, rect) in layout.iter().enumerate() {
            assert_eq!(*rect, expected_sizes[i], "Element {} size mismatch", i);
        }
    }
}
