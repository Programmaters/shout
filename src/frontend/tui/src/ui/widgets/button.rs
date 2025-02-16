use color_eyre::owo_colors::OwoColorize;
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Rect};
use ratatui::prelude::Stylize;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Paragraph, Widget};

pub struct Button {
    pub label: String,
    pub is_pressed: bool,
    pub style: Style,
}

impl Button {
    pub fn click(&mut self) {
        self.is_pressed = !self.is_pressed;
    }
}

impl Widget for Button {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let style = if self.is_pressed {
            self.style.bold()
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
