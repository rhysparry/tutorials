use anyhow::Result;
use crossterm::{
    event::{self, Event::Key, KeyCode::Char},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::{CrosstermBackend, Frame, Terminal},
    widgets::Paragraph,
};

struct App {
    counter: i64,
    should_quit: bool,
}

fn startup() -> Result<()> {
    enable_raw_mode()?;
    execute!(std::io::stderr(), EnterAlternateScreen)?;
    Ok(())
}

fn shutdown() -> Result<()> {
    execute!(std::io::stderr(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn ui(app: &App, f: &mut Frame) {
    let size = f.size();
    let text = format!("Counter: {}", app.counter);
    let paragraph = Paragraph::new(text);
    f.render_widget(paragraph, size);
}

fn update(app: &mut App) -> Result<()> {
    // Check for user input every 250 milliseconds
    if event::poll(std::time::Duration::from_millis(250))? {
        // If a key event occurs, handle it
        if let Key(key) = crossterm::event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    Char('j') => app.counter += 1,
                    Char('k') => app.counter -= 1,
                    Char('q') => app.should_quit = true,
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

fn run() -> Result<()> {
    let mut t = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    let mut app = App {
        counter: 0,
        should_quit: false,
    };

    loop {
        t.draw(|f| ui(&app, f))?;
        update(&mut app)?;
        if app.should_quit {
            break;
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    startup()?;
    let status = run();
    shutdown()?;
    status?;
    Ok(())
}
