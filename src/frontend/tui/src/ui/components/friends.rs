use crate::app::App;
use crate::screens::friends::FriendsScreen;
use ratatui::layout::Rect;
use ratatui::prelude::Text;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

pub fn render_friends(_app: &App, _friends: &FriendsScreen, frame: &mut Frame, area: Rect) {
    let friends_text = Text::from("This is the friends screen");
    let paragraph = Paragraph::new(friends_text);
    frame.render_widget(paragraph, area);
}
