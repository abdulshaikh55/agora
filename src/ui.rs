use std::rc::Rc;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols,
    text::{Line, Text},
    widgets::{Block, Borders, Clear, List, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, CurrentScreen, CurrentlyEditing};
use crate::task_management::{Priority, StatefulList, Status, TaskManager};

pub fn ui(
    frame: &mut Frame,
    app: &App,
    task_manager: &mut TaskManager,
    list_with_state: &mut StatefulList,
) {
    let layout: Rc<[Rect]> = create_main_layout(frame.size());

    render_title(frame, layout[0]);
    render_list(frame, layout[1], list_with_state);

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
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
    }

    if let CurrentScreen::Task = app.current_screen {
        if list_with_state.state.selected() == None {
            let area = centered_rect(30, 10, frame.size());
            let error_string = Paragraph::new("No Task selected")
                .alignment(ratatui::layout::Alignment::Center)
                .block(Block::default().borders(Borders::TOP | Borders::BOTTOM))
                .fg(Color::Red);

            frame.render_widget(error_string, area);
        } else {
            let area = centered_rect(50, 50, frame.size());

            let idx = list_with_state.state.selected().unwrap();
            let task_string = list_with_state.extract_specific_task_string_only(idx);

            let popup_task_layout = Layout::new(
                Direction::Vertical,
                [Constraint::Length(4), Constraint::Length(4)],
            )
            .split(area);

            let second_section = Layout::new(
                Direction::Horizontal,
                [Constraint::Percentage(50), Constraint::Percentage(50)],
            )
            .split(popup_task_layout[1]);

            let new_task_block = Block::default()
                .title("Task")
                .borders(Borders::ALL)
                .border_set(symbols::border::ROUNDED);

            let task_display = Paragraph::new(task_string)
                .block(new_task_block)
                .fg(Color::Yellow)
                .bg(Color::Black);

            frame.render_widget(task_display, popup_task_layout[0]);

            let new_priority_block = Block::default()
                .title("Priority")
                .borders(Borders::ALL)
                .border_set(symbols::border::ROUNDED);

            frame.render_widget(new_priority_block.clone(), second_section[0]);

            let idx = list_with_state.state.selected().unwrap();

            let priority = list_with_state.extract_specific_priority_only(idx);

            let display_priority = match priority {
                Priority::Urgent => Paragraph::new("Urgent")
                    .alignment(Alignment::Center)
                    .bg(Color::Red),

                Priority::Important => Paragraph::new("Important")
                    .alignment(Alignment::Center)
                    .bg(Color::Blue),

                Priority::Normal => Paragraph::new("Normal")
                    .alignment(Alignment::Center)
                    .bg(Color::Green),
            }
            .fg(Color::Black)
            .block(new_priority_block);

            frame.render_widget(display_priority, second_section[0]);

            let new_status_block = Block::default()
                .title("Status")
                .borders(Borders::ALL)
                .border_set(symbols::border::ROUNDED);

            frame.render_widget(new_status_block.clone(), second_section[1]);

            let idx = list_with_state.state.selected().unwrap();

            let status = list_with_state.extract_specific_status_only(idx);

            let display_status = match status {
                Status::NotStarted => Paragraph::new("Not Started")
                    .alignment(Alignment::Center)
                    .fg(Color::Black)
                    .bg(Color::Red)
                    .block(Block::new()),

                Status::Ongoing => Paragraph::new("Ongoing")
                    .alignment(Alignment::Center)
                    .fg(Color::Black)
                    .bg(Color::Blue)
                    .block(Block::default()),

                Status::Completed => Paragraph::new("Complete")
                    .alignment(Alignment::Center)
                    .fg(Color::Black)
                    .bg(Color::Green)
                    .block(Block::default()),
            }
            .block(new_status_block);

            frame.render_widget(display_status, second_section[1]);
        }
    }

    if let CurrentScreen::Editing = app.current_screen {
        if list_with_state.state.selected() == None {
            let area = centered_rect(30, 10, frame.size());
            let error_string = Paragraph::new("No Task selected to edit")
                .alignment(ratatui::layout::Alignment::Center)
                .block(Block::default().borders(Borders::TOP | Borders::BOTTOM))
                .fg(Color::Red);

            frame.render_widget(error_string, area);
        } else {
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
        }
    }

    if let CurrentScreen::Exiting = app.current_screen {
        // frame.render_widget(Clear, frame.size()); // this clears the entire screen and anything already drawn
        let popup_block = Block::default()
            .borders(Borders::ALL)
            .border_set(symbols::border::DOUBLE)
            .style(Style::default().fg(ratatui::style::Color::Red));

        let exit_paragraph = Paragraph::new(" Do you want to exit Task Manager?")
            .block(popup_block)
            .wrap(Wrap { trim: false });

        let area = centered_rect(40, 25, frame.size());
        frame.render_widget(exit_paragraph, area);
    }

    if let CurrentScreen::Delete = app.current_screen {
        // frame.render_widget(Clear, frame.size()); // this clears the entire screen and anything already drawn
        let popup_block = Block::default()
            .borders(Borders::ALL)
            .border_set(symbols::border::DOUBLE)
            .style(Style::default().fg(ratatui::style::Color::Red));

        let exit_paragraph = Paragraph::new(" Do you want to delete this task?")
            .block(popup_block)
            .wrap(Wrap { trim: false });

        let area = centered_rect(40, 25, frame.size());
        frame.render_widget(exit_paragraph, area);
    }
}

/// creates the main layout of three blocks vertically
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

/// renders a block with the title **Task Manager**
fn render_title(frame: &mut Frame, area: Rect) {
    let bottom_border_set = symbols::border::Set {
        bottom_right: symbols::line::NORMAL.vertical_left,
        bottom_left: symbols::line::NORMAL.vertical_right,
        ..symbols::border::ROUNDED
    };
    let title_block = Block::default()
        .borders(Borders::ALL)
        .border_set(bottom_border_set);
    let title_style = Style::default().fg(Color::Green).bold();
    let title = Paragraph::new(Text::styled("Task Manager", title_style))
        .block(title_block)
        .alignment(ratatui::layout::Alignment::Center);

    frame.render_widget(title, area);
}

/// renders the list of tasks that is stateful. i.e. selectable using ListState
fn render_list(frame: &mut Frame, area: Rect, list_with_state: &mut StatefulList) {
    let list_block = Block::default()
        .borders(Borders::LEFT | Borders::RIGHT)
        .white();
    let list_style = Style::default().fg(Color::Cyan);
    let list = List::new(list_with_state.extract_task_string_only())
        .block(list_block)
        .style(list_style)
        .highlight_style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("[*]");

    frame.render_stateful_widget(list, area, &mut list_with_state.state);
}

/// Footer section has two sections
/// 1. **navigation**: indicates which block we're on
/// 2. **controls**: indicates how to maneuver through that block
fn render_footer(frame: &mut Frame, area: Rc<[Rect]>, app: &App) {
    let top_left_border_set = symbols::border::Set {
        top_left: symbols::line::NORMAL.vertical_right,
        ..symbols::border::ROUNDED
    };

    let navigation = match app.current_screen {
        CurrentScreen::Main => Line::styled(" Main Menu", Style::default().fg(Color::Gray)),
        CurrentScreen::Editing => Line::styled(" Editing", Style::default().fg(Color::Green)),
        CurrentScreen::Task => Line::styled(" Task View", Style::default().fg(Color::Yellow)),
        CurrentScreen::Exiting => Line::styled(" Exiting", Style::default().fg(Color::Red)),
        CurrentScreen::New => Line::styled(" New Task", Style::default().fg(Color::Blue)),
        CurrentScreen::Delete => Line::styled(" Delete", Style::default().fg(Color::Red)),
    };
    let navigation = Paragraph::new(navigation).block(
        Block::default()
            .borders(Borders::LEFT | Borders::TOP | Borders::BOTTOM)
            .border_set(top_left_border_set),
    );

    let top_right_border_set = symbols::border::Set {
        top_left: symbols::line::NORMAL.horizontal_down,
        bottom_left: symbols::line::NORMAL.horizontal_up,
        top_right: symbols::line::NORMAL.vertical_left,
        ..symbols::border::ROUNDED
    };
    frame.render_widget(navigation, area[0]);

    let control_panel = match app.current_screen {
        CurrentScreen::Main => Line::styled(
            " [➡] / [⬅] select / unselect, [Enter] for new, [Esc] to exit",
            Style::default().fg(Color::Green),
        ),
        CurrentScreen::Editing => Line::styled(
            " [⬆] / [⬇] to move, [Tab] to toggle values, [Enter] to confirm, [Esc] to go back",
            Style::default().fg(Color::Green),
        ),
        CurrentScreen::Task => Line::styled(
            " [➡] to Edit [Esc] to go back",
            Style::default().fg(Color::Green),
        ),
        CurrentScreen::Exiting => Line::styled(
            " [y] for yes, [n] for no",
            Style::default().fg(Color::Green),
        ),
        CurrentScreen::New => Line::styled(
            " [Esc] to go back, [Enter] to continue, [Arrow] to toggle aspects, [Tab] to change value",
            Style::default().fg(Color::Green),
        ),
        CurrentScreen::Delete => Line::styled(
            " [y] for yes, [n] for no",
            Style::default().fg(Color::Green),
        ),
    };
    let control_panel = Paragraph::new(control_panel).block(
        Block::default()
            .borders(Borders::ALL)
            .border_set(top_right_border_set),
    );
    frame.render_widget(control_panel, area[1]);
}

/// renders a block that takes string as input which will be the task
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

    frame.render_widget(Clear, area);
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

/// renders a block that toggles between priority enums {Urgent, Important, Normal}
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
            .alignment(Alignment::Center)
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
            .alignment(Alignment::Center)
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
            .alignment(Alignment::Center)
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

/// renders a block that toggles between Status enums {NotStarted, Ongoing, Completed}
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
            .alignment(Alignment::Center)
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
            .alignment(Alignment::Center)
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
            .alignment(Alignment::Center)
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
        let rect = Rect::new(0, 0, 200, 150);

        let expected_rect = Rect::new(74, 18, 50, 113);
        let actual_rect = centered_rect(25, 75, rect);
        assert_eq!(actual_rect, expected_rect);
    }

    #[test]
    fn test_create_main_layout() {
        let screen_size = Rect::new(0, 0, 80, 25);

        let layout: Rc<[Rect]> = create_main_layout(screen_size);

        let expected_sizes: Rc<[Rect]> = [
            // Title: 3 units of height (assuming full width)
            Rect::new(0, 0, 80, 3),
            // List: Min height of 2 (assuming full width)
            Rect::new(0, 3, 80, 19),
            // Footer: 3 units of height (assuming full width)
            Rect::new(0, 22, 80, 3),
        ]
        .into();

        for (i, rect) in layout.iter().enumerate() {
            assert_eq!(*rect, expected_sizes[i], "Element {} size mismatch", i);
        }
    }
}
