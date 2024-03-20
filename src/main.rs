mod app;
mod controls;
mod task_management;
mod ui;

use app::{App, CurrentScreen};
use controls::StatefulList;
use task_management::{Task, TaskManager};

use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{event, execute};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
//use std::fs::File;
use std::io::stderr;
// use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    enable_raw_mode()?;
    execute!(stderr(), EnterAlternateScreen)?;

    let mut stderr = stderr();
    execute!(stderr, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

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
    // vec![
    //     "Eat".to_string(),
    //     "Code".to_string(),
    //     "Sleep".to_string(),
    //     "Repeat".to_string(),
    // ];

    let mut app = App::new();
    let mut task_manager = TaskManager::new();
    task_manager.insert_tasks(tasks);
    let mut task_with_state = StatefulList::new(&task_manager.tasks);

    // // write data into tasks.json
    // let data = serde_json::to_string(&task_manager)?;
    // let mut file = File::create("tasks.json")?;
    // file.write_all(data.as_bytes())?;

    // let mut file = File::open("tasks.json")?;
    // let mut file_content = String::new();
    // file.read_to_string(&mut file_content)?;
    //let tasks: Vec<String> = serde_json::from_str(&file_content)?;

    loop {
        terminal.draw(|f| ui::ui(f, &mut task_with_state, &app, &task_manager))?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release {
                    continue;
                }
                match app.current_screen {
                    CurrentScreen::Main => match key.code {
                        // Go to exit screen when [Esc] is pressed
                        KeyCode::Esc => app.change_screen(CurrentScreen::Exiting),
                        // Go to new screen when [=] is pressed
                        KeyCode::Enter => app.change_screen(CurrentScreen::New),
                        // Task list navigation section
                        KeyCode::Right => app.change_screen(CurrentScreen::Task),
                        KeyCode::Left => task_with_state.unselect(),
                        KeyCode::Up => task_with_state.previous(),
                        KeyCode::Down => task_with_state.next(),
                        _ => (),
                    },
                    CurrentScreen::New => match key.code {
                        KeyCode::Esc => app.change_screen(CurrentScreen::Main),
                        KeyCode::Char(c) => task_manager.input_task.push(c),
                        KeyCode::Backspace => _ = task_manager.input_task.pop().unwrap(),
                        _ => (),
                    },
                    CurrentScreen::Task => match key.code {
                        KeyCode::Left => app.change_screen(CurrentScreen::Main),
                        KeyCode::Right => app.change_screen(CurrentScreen::Editing),
                        _ => (),
                    },
                    CurrentScreen::Editing => match key.code {
                        KeyCode::Left => app.change_screen(CurrentScreen::Task),
                        // KeyCode::Char(c) => task_manager.tasks,
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

    // todo!("write data");

    Ok(())
}
