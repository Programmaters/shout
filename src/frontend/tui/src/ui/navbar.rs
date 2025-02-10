use ratatui::prelude::{Color, Line, Span, Style};
use ratatui::style::Stylize;
use crate::app::App;
use crate::models::screen::Screen;

pub fn navbar(app: &App) -> Line {
    let mut nav_items: Vec<Span> = Vec::new();
    for screen in Screen::all() {
        if screen == app.screen {
            nav_items.push(Span::styled(
                format!(" {} ", screen.as_str()),
                Style::default().fg(Color::Yellow).bold(),
            ));
        } else {
            nav_items.push(Span::raw(format!(" {} ", screen.as_str())));
        }
        nav_items.push(Span::raw("|"));
    }
    nav_items.pop(); // remove last "|"
    Line::from(nav_items)
}
