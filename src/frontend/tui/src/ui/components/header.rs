use std::mem::discriminant;
use crate::app::App;
use crate::models::screen::Screen;
use ratatui::layout::{Alignment, Rect};
use ratatui::prelude::{Line, Span, Style};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Paragraph};
use ratatui::Frame;
use crate::ui::utils::select_state::SelectState;

pub fn header(app: &App, frame: &mut Frame, area: Rect) {
    let main_block = Block::bordered();
    let title_content = Paragraph::new(" Shout ")
        .block(main_block.clone())
        .alignment(Alignment::Left);

    let navbar_content = Paragraph::new(navbar(&app.screen))
        .block(main_block)
        .alignment(Alignment::Right);

    frame.render_widget(title_content, area);
    frame.render_widget(navbar_content, area);
}

fn navbar(curr_screen: &Screen) -> Line {
    let mut nav_items: Vec<Span> = Vec::new();
    for screen in Screen::all() {
        let in_screen = discriminant(&screen) == discriminant(curr_screen);
        nav_items.push(Span::styled(
            format!(" {} ", screen.as_str()),
            Style::default().fg(SelectState::from_bool(in_screen).to_color()).bold(),
        ));
        nav_items.push(Span::raw("|"));
    }
    nav_items.pop(); // remove last "|"
    Line::from(nav_items)
}
