mod navbar;
mod footer;
mod main_content;

use crate::app::App;
use crate::ui::footer::footer;
use crate::ui::main_content::main_content;
use ratatui::style::Stylize;
use ratatui::{layout::{Constraint, Direction, Layout}, Frame};

pub fn ui(frame: &mut Frame, app: &App) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0), // main content
            Constraint::Length(3), // footer
        ])
        .split(frame.area());

    let main_content = main_content(app);
    let footer = footer(app);

    // render components
    frame.render_widget(main_content, layout[0]);
    frame.render_widget(footer, layout[1]);
}
