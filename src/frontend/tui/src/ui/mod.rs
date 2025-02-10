mod components;

use crate::app::App;
use ratatui::{layout::{Constraint, Direction, Layout}, Frame};
use ratatui::layout::Rect;
use crate::models::screen::Screen;
use crate::ui::components::chat::chat;
use crate::ui::components::footer::footer;
use crate::ui::components::friends::friends;
use crate::ui::components::header::header;
use crate::ui::components::profile::profile;

pub fn ui(app: &App, frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // header
            Constraint::Min(0), // main content
            Constraint::Length(3), // footer
        ])
        .split(frame.area());

    header(app, frame, layout[0]);
    main_content(app, frame, layout[1]);
    footer(app, frame, layout[2]);
}

fn main_content(app: &App, frame: &mut Frame, rect: Rect) {
    match app.screen {
        Screen::Profile => profile(app, frame, rect),
        Screen::Chat => chat(app, frame, rect),
        Screen::Friends => friends(app, frame, rect)
    };
}