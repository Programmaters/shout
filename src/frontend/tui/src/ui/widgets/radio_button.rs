use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::Widget,
};
use ratatui::text::Text;

pub struct RadioButton {
    pub label: String,
    pub selected: bool,
    pub style: Style,
}

impl Widget for RadioButton {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let symbol = if self.selected { ">" } else { " " };
        let text = format!("{} {}", symbol, self.label);
        let styled_text = Text::from(text.as_str());
        let paragraph = ratatui::widgets::Paragraph::new(styled_text).style(self.style);
        paragraph.render(area, buf);
    }
}
