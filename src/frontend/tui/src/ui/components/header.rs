use crate::app::App;
use crate::models::screen::Screen;
use ratatui::layout::{Alignment, Rect};
use ratatui::prelude::{Color, Line, Span, Style};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Paragraph};
use ratatui::Frame;

pub fn header(app: &App, frame: &mut Frame, rect: Rect) {
    let main_block = Block::bordered();
    let title_content = Paragraph::new(" Shout ")
        .block(main_block.clone())
        .alignment(Alignment::Left);

    let navbar_content = Paragraph::new(navbar(app))
        .block(main_block)
        .alignment(Alignment::Right);

    frame.render_widget(title_content, rect);
    frame.render_widget(navbar_content, rect);
}

fn navbar(app: &App) -> Line {
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
