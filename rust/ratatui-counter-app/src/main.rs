use crossterm::{
    event::{self, Event::Key, KeyCode::Char},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::{CrosstermBackend, Frame, Terminal},
    widgets::Paragraph,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // startup: Enable raw mode for the terminal, giving us fine control over user input
    enable_raw_mode()?;
    execute!(std::io::stderr(), EnterAlternateScreen)?;

    // Initialize the terminal backend using crossterm
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    // Define our counter variable
    // This is the state of our application
    let mut counter = 0;

    // Main application loop
    loop {
        // Render the UI
        terminal.draw(|f| {
            f.render_widget(Paragraph::new(format!("Counter: {counter}")), f.size());
        })?;

        // Check for user input every 250 milliseconds
        if event::poll(std::time::Duration::from_millis(250))? {
            // If a key event occurs, handle it
            if let Key(key) = crossterm::event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        Char('j') => counter += 1,
                        Char('k') => counter -= 1,
                        Char('q') => break,
                        _ => {}
                    }
                }
            }
        }
    }

    // shutdown down: reset terminal back to original state
    execute!(std::io::stderr(), LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
