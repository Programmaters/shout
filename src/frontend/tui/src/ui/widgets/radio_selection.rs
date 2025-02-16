use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style},
    widgets::{Block, Borders, Widget},
};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::text::Text;
use crate::ui::widgets::button::Button;
use crate::ui::widgets::radio_button::RadioButton;

pub struct RadioSelection<'a> {
    pub options: Vec<&'a str>,
    pub selected_index: usize,
    pub block: Block<'a>,
}

impl<'a> Widget for RadioSelection<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let buttons = self.options
            .iter()
            .enumerate()
            .map(|(idx, p)| {
                RadioButton {
                    label: p.to_string(),
                    selected: idx == self.selected_index,
                    style: Default::default(),
                }
            })
            .collect::<Vec<_>>();

        let constraints = buttons
            .iter()
            .map(|_| Constraint::Length(1))
            .collect::<Vec<_>>();

        let inner_area = self.block.inner(area);
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(inner_area);

        self.block.render(area, buf);
        for (i, button) in buttons.into_iter().enumerate() {
            button.render(chunks[i], buf);
        }
    }
}