pub mod app;
pub mod event;
pub mod tui;
pub mod ui;
pub mod update;

use app::App;
use color_eyre::Result;
use event::{Event, EventHandler};
use ratatui::{backend::CrosstermBackend, Terminal};
use tui::Tui;
use update::update;

fn main() -> Result<()> {
    // Create an application
    let mut app = App::new();

    // Initialize the terminal user interface
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    // Start the main loop
    while !app.should_quit() {
        // Render the user interface.
        tui.draw(&app)?;
        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => update(&mut app, key_event),
            Event::Mouse(_mouse_event) => {}
            Event::Resize(_width, _height) => {}
        };
    }

    tui.exit()?;

    Ok(())
}
