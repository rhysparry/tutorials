use crate::app::{App, CurrentScreen, CurrentlyEditing};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap};
use ratatui::Frame;
use std::rc::Rc;

struct AppLayout {
    chunks: Rc<[Rect]>,
}

impl AppLayout {
    fn new(frame: &Frame) -> Self {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(3),
            ])
            .split(frame.size());

        Self { chunks }
    }

    fn header(&self) -> Rect {
        self.chunks[0]
    }

    fn body(&self) -> Rect {
        self.chunks[1]
    }

    fn footer(&self) -> Rect {
        self.chunks[2]
    }

    fn render_title(&self, frame: &mut Frame) {
        let title_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());

        let title = Paragraph::new(Text::styled(
            "Create new JSON",
            Style::default().fg(Color::Green),
        ))
        .block(title_block);

        frame.render_widget(title, self.header());
    }

    fn render_pairs(&self, frame: &mut Frame, app: &App) {
        // The list of existing pairs
        let mut list_items = Vec::<ListItem>::new();

        for key in app.pairs.keys() {
            list_items.push(ListItem::new(Line::from(Span::styled(
                format!("{: <25} : {}", key, app.pairs.get(key).unwrap()),
                Style::default().fg(Color::White),
            ))));
        }

        let list = List::new(list_items);

        frame.render_widget(list, self.body());
    }

    fn get_mode_footer(app: &App) -> Paragraph {
        let current_navigation_text = vec![
            // The first half of the text
            match app.current_screen {
                CurrentScreen::Main => {
                    Span::styled("Normal Mode", Style::default().fg(Color::Green))
                }
                CurrentScreen::Editing => {
                    Span::styled("Editing Mode", Style::default().fg(Color::Yellow))
                }
                CurrentScreen::Exiting => {
                    Span::styled("Exiting", Style::default().fg(Color::LightRed))
                }
            }
            .to_owned(),
            // A white divider bar to separate the two sections
            Span::styled(" | ", Style::default().fg(Color::White)),
            // The final section of the text, with hints on what the user is editing
            {
                if let Some(editing) = &app.currently_editing {
                    match editing {
                        CurrentlyEditing::Key => {
                            Span::styled("Editing Json Key", Style::default().fg(Color::Green))
                        }
                        CurrentlyEditing::Value => Span::styled(
                            "Editing Json Value",
                            Style::default().fg(Color::LightGreen),
                        ),
                    }
                } else {
                    Span::styled("Not Editing Anything", Style::default().fg(Color::DarkGray))
                }
            },
        ];

        Paragraph::new(Line::from(current_navigation_text))
            .block(Block::default().borders(Borders::ALL))
    }

    fn get_keys_hint_footer(app: &App) -> Paragraph {
        let current_keys_hint = {
            match app.current_screen {
                CurrentScreen::Main => Span::styled(
                    "(q) to quit / (e) to make new pair",
                    Style::default().fg(Color::Red),
                ),
                CurrentScreen::Editing => Span::styled(
                    "(ESC) to cancel/(Tab) to switch boxes/enter to complete",
                    Style::default().fg(Color::Red),
                ),
                CurrentScreen::Exiting => Span::styled(
                    "(q) to quit / (e) to make new pair",
                    Style::default().fg(Color::Red),
                ),
            }
        };

        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL))
    }

    fn render_footer(&self, frame: &mut Frame, app: &App) {
        let footer_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(self.footer());

        let mode_footer = Self::get_mode_footer(app);
        let keys_hint_footer = Self::get_keys_hint_footer(app);
        frame.render_widget(mode_footer, footer_chunks[0]);
        frame.render_widget(keys_hint_footer, footer_chunks[1]);
    }

    fn render_editing_popup(&self, frame: &mut Frame, app: &App) {
        if let Some(editing) = &app.currently_editing {
            let popup_layout = PopupLayout::new(frame);
            popup_layout.render(frame, app, editing);
        }
    }

    fn render_exiting_popup(&self, frame: &mut Frame, app: &App) {
        if let CurrentScreen::Exiting = app.current_screen {
            frame.render_widget(Clear, frame.size()); // Clear the screen

            let popup_block = Block::default()
                .title("Y/N")
                .borders(Borders::NONE)
                .style(Style::default().bg(Color::DarkGray));

            let exit_text = Text::styled(
                "Would you like to output the buffer as JSON? (y/n)",
                Style::default().fg(Color::Red),
            );

            // The `trim: false` will stop the text from being cut off when over the edge of the block
            let exit_paragraph = Paragraph::new(exit_text)
                .block(popup_block)
                .wrap(Wrap { trim: false });

            let area = centered_rect(60, 25, frame.size());
            frame.render_widget(exit_paragraph, area);
        }
    }

    fn render(&self, frame: &mut Frame, app: &App) {
        self.render_title(frame);
        self.render_pairs(frame, app);
        self.render_footer(frame, app);
        self.render_editing_popup(frame, app);
        self.render_exiting_popup(frame, app);
    }
}

struct PopupLayout {
    area: Rect,
    chunks: Rc<[Rect]>,
    active_style: Style,
}

impl PopupLayout {
    fn new(frame: &Frame) -> Self {
        let area = centered_rect(60, 25, frame.size());
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        Self {
            area,
            chunks,
            active_style: Style::default().bg(Color::LightYellow).fg(Color::Black),
        }
    }

    fn render_block(&self, frame: &mut Frame) {
        let popup_block = Block::default()
            .title("Enter a new key-value pair")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));
        frame.render_widget(popup_block, self.area)
    }

    fn render_key_block(&self, frame: &mut Frame, app: &App, editing: &CurrentlyEditing) {
        let mut key_block = Block::default().title("Key").borders(Borders::ALL);

        if let CurrentlyEditing::Key = editing {
            key_block = key_block.style(self.active_style);
        }

        let key_text = Paragraph::new(app.key_input.clone()).block(key_block);
        frame.render_widget(key_text, self.chunks[0]);
    }

    fn render_value_block(&self, frame: &mut Frame, app: &App, editing: &CurrentlyEditing) {
        let mut value_block = Block::default().title("Value").borders(Borders::ALL);

        if let CurrentlyEditing::Value = editing {
            value_block = value_block.style(self.active_style);
        }

        let value_text = Paragraph::new(app.value_input.clone()).block(value_block);
        frame.render_widget(value_text, self.chunks[1]);
    }

    fn render(&self, frame: &mut Frame, app: &App, editing: &CurrentlyEditing) {
        self.render_block(frame);
        self.render_key_block(frame, app, editing);
        self.render_value_block(frame, app, editing);
    }
}

pub fn ui(f: &mut Frame, app: &App) {
    let layout = AppLayout::new(f);
    layout.render(f, app);
}

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

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
