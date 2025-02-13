use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::Text;
use ratatui::widgets::Paragraph;
use crate::app::App;
use crate::models::screen::FriendsScreen;

pub fn render_friends(_app: &App, _friends: &FriendsScreen, frame: &mut Frame, area: Rect) {
    let friends_text = Text::from("This is the friends screen");
    let paragraph = Paragraph::new(friends_text);
    frame.render_widget(paragraph, area);
}