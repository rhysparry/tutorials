use ratatui::{
    prelude::{Alignment, Frame},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::App;

pub fn render(app: &App, f: &mut Frame) {
    f.render_widget(
        Paragraph::new(format!(
            "\
              Press `Esc`. `Ctrl+C` or `q` to quit.\n\
              Press `j` or `k` to increment or decrement the counter respectively.\n\
              Counter: {}
            ",
            app.counter()
        ))
        .block(
            Block::default()
                .title("Counter App")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center),
        f.size(),
    )
}
