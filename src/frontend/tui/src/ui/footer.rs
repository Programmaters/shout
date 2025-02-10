use ratatui::layout::Alignment;
use ratatui::prelude::{Color, Line, Span, Style};
use ratatui::widgets::{Block, Paragraph};
use crate::app::App;

pub fn footer(_app: &App) -> Paragraph {
    let help_text = Line::from(vec![
        Span::raw(" Quit: "),
        Span::styled("Ctrl+C", Style::default().fg(Color::Yellow)),
        Span::raw(" | Switch Screen: "),
        Span::styled("Ctrl+←/→", Style::default().fg(Color::Cyan)),
    ]);
    Paragraph::new(help_text)
        .alignment(Alignment::Center)
        .block(Block::default())
}