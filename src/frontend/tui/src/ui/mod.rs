mod components;

use crate::app::App;
use crate::models::screen::Screen;
use crate::ui::components::chat::render_chat;
use crate::ui::components::footer::footer;
use crate::ui::components::friends::render_friends;
use crate::ui::components::header::header;
use crate::ui::components::profile::render_profile;
use ratatui::{layout::{Constraint, Direction, Layout}, Frame};
use ratatui::layout::Rect;

pub fn render_ui(app: &App, frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // header
            Constraint::Min(0), // main content
            Constraint::Length(3), // footer
        ])
        .split(frame.area());

    header(app, frame, layout[0]);
    render_content(app, frame, layout[1]);
    footer(app, frame, layout[2]);
}

fn render_content(app: &App, frame: &mut Frame, area: Rect) {
    match app.screen {
        Screen::Profile => render_profile(app, frame, area),
        Screen::Chat => render_chat(app, frame, area),
        Screen::Friends => render_friends(app, frame, area)
    };
}