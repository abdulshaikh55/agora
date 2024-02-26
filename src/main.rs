mod ui;
mod introduction;
mod app;
mod controls;

use introduction::introduction_frame;
use app::{App, CurrentScreen};
use controls::StatefulList;

use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{event, execute};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::fs::File;
use std::io::{stderr, Read, Write};

fn main() -> std::io::Result<()> {
    enable_raw_mode()?;
    execute!(stderr(), EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stderr());
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(introduction_frame)?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        KeyCode::Enter => break,
                        _ => (),
                    }
                }
            }
        }
    }

    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    let mut stderr = stderr();
    execute!(stderr, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let tasks = vec!["Eat".to_string(), "Code".to_string(), "Sleep".to_string(), "Repeat".to_string()];

    // write data into tasks.json
    let data = serde_json::to_string(&tasks)?;
    let mut file = File::create("tasks.json")?;
    file.write_all(data.as_bytes())?;

    // read data into the instance of list_with_states
    let mut file = File::open("tasks.json")?;
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;

    let tasks: Vec<String> = serde_json::from_str(&file_content)?;
    let mut list_with_state = StatefulList::new(tasks);

    let mut app = App::new();

    loop {
        terminal.draw(|f| ui::ui(f, &mut list_with_state, &app))?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release{
                    continue;
                } 
                match app.current_screen {
                    CurrentScreen::Main => match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => app.current_screen = CurrentScreen::Exiting,
                        KeyCode::Right => list_with_state.select(),
                        KeyCode::Left => list_with_state.unselect(),
                        KeyCode::Up => list_with_state.previous(),
                        KeyCode::Down => list_with_state.next(),
                        _ => (),
                    },
                    CurrentScreen::Task => todo!("After selecting the task, edit the various attributes"),
                    CurrentScreen::Exiting => match key.code {
                        KeyCode::Char('y') => break,
                        KeyCode::Char('n') => {
                            app.current_screen = CurrentScreen::Main;
                            continue;
                        },
                        _ => (),
                    },
                    CurrentScreen::Editing => ()
                }


            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}