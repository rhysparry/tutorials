/// Application
#[derive(Debug, Default)]
pub struct App {
    /// counter
    counter: u8,
    /// should the application exit?
    should_quit: bool,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    pub fn counter(&self) -> u8 {
        self.counter
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    /// Handles the tick event of the terminal
    pub fn tick(&self) {}

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn increment_counter(&mut self) {
        if let Some(result) = self.counter.checked_add(1) {
            self.counter = result;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(result) = self.counter.checked_sub(1) {
            self.counter = result;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_increment_counter() {
        let mut app = App::new();
        app.increment_counter();
        assert_eq!(app.counter, 1);
    }

    #[test]
    fn test_app_decrement_counter() {
        let mut app = App::new();
        app.decrement_counter();
        assert_eq!(app.counter, 0);
    }
}
