use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::Text;
use ratatui::widgets::Paragraph;
use crate::app::App;

pub fn profile(_app: &App, frame: &mut Frame, rect: Rect) {
    let profile_text = Text::from("This is the profile screen");
    let paragraph = Paragraph::new(profile_text);
    frame.render_widget(paragraph, rect);
}
