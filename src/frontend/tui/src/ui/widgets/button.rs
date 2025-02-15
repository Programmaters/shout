use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Paragraph, Widget};

pub struct Button {
    pub label: String,
    pub is_pressed: bool,
    pub style: Style,
    pub pressed_style: Option<Style>,
}

impl Widget for Button {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let style = if self.is_pressed {
            self.pressed_style
                .unwrap_or_else(|| Style::default().fg(Color::Blue))
        } else {
            self.style
        };
        let paragraph = Paragraph::new(self.label)
            .style(style)
            .alignment(Alignment::Center)
            .block(Block::bordered());
        paragraph.render(area, buf);
    }
}
