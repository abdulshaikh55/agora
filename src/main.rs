mod app;
mod task_management;
mod ui;

use app::{App, CurrentScreen, CurrentlyEditing};
use task_management::{StatefulList, Task, TaskManager};

use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{event, execute};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::fs::File;
use std::io::stderr;
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    // 🚀 Prepare to enter alternate screen
    enable_raw_mode()?;
    execute!(stderr(), EnterAlternateScreen)?;

    let mut stderr = stderr();
    execute!(stderr, EnterAlternateScreen)?;

    // create the terminal on backend
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // read data from .json file.
    let mut file = File::open("tasks.json")?;
    let mut file_content = String::new();
    let mut tasks: Vec<Task> = Vec::new();
    if file.read_to_string(&mut file_content)? != 0 {
        tasks = serde_json::from_str(&file_content)?;
    }

    let mut app = App::new();
    let mut task_manager = TaskManager::new(tasks);
    let mut task_with_state = StatefulList::new(&task_manager.tasks);

    loop {
        terminal.draw(|f| ui::ui(f, &app, &task_manager, &mut task_with_state))?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release {
                    continue;
                }
                match app.current_screen {
                    CurrentScreen::Main => match key.code {
                        // Go to exit screen when [Esc] is pressed
                        KeyCode::Esc => app.change_screen(CurrentScreen::Exiting),
                        KeyCode::Enter => app.change_screen(CurrentScreen::New),
                        // Task list navigation section
                        KeyCode::Right => app.change_screen(CurrentScreen::Task),
                        KeyCode::Left => task_with_state.unselect(),
                        KeyCode::Up => task_with_state.previous(),
                        KeyCode::Down => task_with_state.next(),
                        _ => (),
                    },
                    CurrentScreen::New => match app.currently_editing {
                        CurrentlyEditing::Task => match key.code {
                            KeyCode::Esc => app.change_screen(CurrentScreen::Main),
                            KeyCode::Char(c) => task_manager.input_task_string.push(c),
                            KeyCode::Backspace => {
                                if !task_manager.input_task_string.is_empty() {
                                    task_manager.input_task_string.pop().unwrap();
                                }
                            }
                            KeyCode::Up | KeyCode::Down => app.toggle_task_priority(),
                            KeyCode::Right | KeyCode::Left => app.toggle_priority_status(),
                            KeyCode::Enter => {
                                if !task_manager.input_task_string.is_empty() {
                                    task_manager.save_instance_task();
                                    task_with_state = StatefulList::new(&task_manager.tasks);
                                }
                                app.change_screen(CurrentScreen::Main)
                            }
                            _ => (),
                        },
                        CurrentlyEditing::Priority => match key.code {
                            KeyCode::Esc => app.change_screen(CurrentScreen::Main),
                            KeyCode::Up | KeyCode::Down => app.toggle_task_priority(),
                            KeyCode::Right | KeyCode::Left => app.toggle_priority_status(),
                            KeyCode::Tab => task_manager.switch_priority_value(),
                            KeyCode::Enter => {
                                if !task_manager.input_task_string.is_empty() {
                                    task_manager.save_instance_task();
                                    task_with_state = StatefulList::new(&task_manager.tasks);
                                }
                                app.change_screen(CurrentScreen::Main)
                            }
                            _ => (),
                        },
                        CurrentlyEditing::Status => match key.code {
                            KeyCode::Esc => app.change_screen(CurrentScreen::Main),
                            KeyCode::Up | KeyCode::Down => app.toggle_task_priority(),
                            KeyCode::Right | KeyCode::Left => app.toggle_priority_status(),
                            KeyCode::Tab => task_manager.switch_status_value(),
                            KeyCode::Enter => {
                                if !task_manager.input_task_string.is_empty() {
                                    task_manager.save_instance_task();
                                    task_with_state = StatefulList::new(&task_manager.tasks);
                                }
                                app.change_screen(CurrentScreen::Main)
                            }
                            _ => (),
                        },
                    },
                    CurrentScreen::Task => match key.code {
                        KeyCode::Left => app.change_screen(CurrentScreen::Main),
                        KeyCode::Right => app.change_screen(CurrentScreen::Editing),
                        _ => (),
                    },
                    CurrentScreen::Editing => match key.code {
                        KeyCode::Left => app.change_screen(CurrentScreen::Task),
                        KeyCode::Down => (),
                        _ => (), // app.change_screen(CurrentScreen::Editing(Some(edit)));
                    },
                    CurrentScreen::Exiting => match key.code {
                        KeyCode::Char('y') => break,
                        KeyCode::Char('n') => {
                            app.change_screen(CurrentScreen::Main);
                            continue;
                        }
                        _ => (),
                    },
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    // write data into tasks.json
    let data = serde_json::to_string(&task_manager.tasks)?;
    let mut file = File::create("tasks.json")?;
    file.write_all(data.as_bytes())?;

    Ok(())
}
