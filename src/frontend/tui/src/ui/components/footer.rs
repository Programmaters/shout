use crate::app::App;
use ratatui::layout::{Alignment, Rect};
use ratatui::prelude::{Color, Line, Span, Style};
use ratatui::widgets::{Block, Paragraph};
use ratatui::Frame;

pub fn render_footer(_app: &App, frame: &mut Frame, area: Rect) {
    let help_text = Line::from(vec![
        Span::raw("Navigate Screen: "),
        Span::styled("←/↑/→/↓", footer_span_style()),
        Span::raw(" | Switch Screen: "),
        Span::styled("Ctrl", footer_span_style()),
        Span::raw("+"),
        Span::styled("←/→", footer_span_style()),
        Span::raw(" | Toggle Popup: "),
        Span::styled("Tab", footer_span_style()),
        Span::raw(" | Quit: "),
        Span::styled("Esc", footer_span_style()),
    ]);
    let paragraph = Paragraph::new(help_text)
        .alignment(Alignment::Center)
        .block(Block::default());

    frame.render_widget(paragraph, area);
}

fn footer_span_style() -> Style {
    Style::default().fg(Color::LightCyan)
}
