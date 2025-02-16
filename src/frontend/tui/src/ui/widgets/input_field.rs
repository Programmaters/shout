use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph, Widget};

pub struct InputField<'a> {
    pub value: String,
    pub placeholder: Option<String>,
    pub style: Style,
    pub block: Block<'a>,
}

impl<'a> Widget for InputField<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let content = if self.value.is_empty() {
            self.placeholder.unwrap_or("".to_string())
        } else {
            self.value.clone()
        };
        let paragraph = Paragraph::new(content)
            .style(self.style)
            .alignment(Alignment::Left)
            .block(self.block);

        paragraph.render(area, buf);
    }
}
