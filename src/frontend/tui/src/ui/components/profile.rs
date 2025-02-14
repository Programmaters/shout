use crate::app::App;
use crate::screens::profile::ProfileScreen;
use ratatui::layout::Rect;
use ratatui::prelude::Text;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

pub fn render_profile(_app: &App, _profile: &ProfileScreen, frame: &mut Frame, area: Rect) {
    let profile_text = Text::from("This is the profile screen");
    let paragraph = Paragraph::new(profile_text);
    frame.render_widget(paragraph, area);
}
