mod ui;
use crossterm::event::{Event, KeyCode};
use ui::StatefulList;

use crossterm::terminal::{EnterAlternateScreen, enable_raw_mode, LeaveAlternateScreen, disable_raw_mode};
use crossterm::{event, execute};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use std::io::stderr;

fn main() -> std::io::Result<()> {
    enable_raw_mode()?;
    // let mut stderr = stderr();
    execute!(stderr(), EnterAlternateScreen)?;
    
    let backend = CrosstermBackend::new(stderr());
    let mut terminal = Terminal::new(backend)?;
        
    loop {
        
        terminal.draw(ui::introduction)?;

        if event::poll(std::time::Duration::from_millis(50))? {
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

    let mut list_with_state = StatefulList::new(tasks);
    
    loop {
        
        terminal.draw(|f| ui::ui(f, &mut list_with_state))?;

        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Left => list_with_state.unselect(),
                        KeyCode::Up => list_with_state.previous(),
                        KeyCode::Down => list_with_state.next(),
                        _ => (),
                    }
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}

