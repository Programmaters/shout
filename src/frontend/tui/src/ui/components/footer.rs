use ratatui::Frame;
use ratatui::layout::{Alignment, Rect};
use ratatui::prelude::{Color, Line, Span, Style};
use ratatui::widgets::{Block, Paragraph};
use crate::app::App;

pub fn render_footer(_app: &App, frame: &mut Frame, area: Rect) {
    let help_text = Line::from(vec![
        Span::raw("Navigate Screen: "),
        Span::styled("←/↑/→/↓", Style::default().fg(Color::Yellow)),
        Span::raw(" | Switch Screen: "),
        Span::styled("Ctrl", Style::default().fg(Color::Yellow)),
        Span::raw("+"),
        Span::styled("←/→", Style::default().fg(Color::Yellow)),
        Span::raw(" | Quit: "),
        Span::styled("Esc", Style::default().fg(Color::Yellow)),
    ]);
    let paragraph = Paragraph::new(help_text)
        .alignment(Alignment::Center)
        .block(Block::default());

    frame.render_widget(paragraph, area);
}