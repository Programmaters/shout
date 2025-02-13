mod components;
mod utils;

use crate::app::App;
use crate::models::screen::Screen;
use crate::ui::components::chat::render_chat;
use crate::ui::components::footer::render_footer;
use crate::ui::components::friends::render_friends;
use crate::ui::components::header::render_header;
use crate::ui::components::profile::render_profile;
use ratatui::{layout::{Constraint, Direction, Layout}, Frame};
use ratatui::layout::Rect;

pub fn render_ui(app: &App, frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // header
            Constraint::Min(0), // main content
            Constraint::Length(1), // footer
        ])
        .split(frame.area());

    render_header(app, frame, layout[0]);
    render_content(app, frame, layout[1]);
    render_footer(app, frame, layout[2]);
}

fn render_content(app: &App, frame: &mut Frame, area: Rect) {
    match app.get_screen() {
        Screen::Profile(profile) => render_profile(app, &profile, frame, area),
        Screen::Chat(chat) => render_chat(app, &chat, frame, area),
        Screen::Friends(friends) => render_friends(app, &friends, frame, area)
    };
}