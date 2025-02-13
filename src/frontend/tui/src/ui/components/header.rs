use std::mem::discriminant;
use crate::app::App;
use crate::models::screen::Screen;
use ratatui::layout::{Alignment, Rect};
use ratatui::prelude::{Color, Line, Modifier, Span, Style};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Paragraph};
use ratatui::Frame;
use crate::ui::utils::select_state::SelectState;

pub fn render_header(app: &App, frame: &mut Frame, area: Rect) {
    let main_block = Block::bordered();
    let title_content = Paragraph::new(" shout-tui ")
        .block(main_block.clone())
        .alignment(Alignment::Left);

    let logged_user = app.logged_user.clone().unwrap();
    let online_status_style = if logged_user.online {
        Style::default()
            .fg(Color::LightGreen)
            .add_modifier(Modifier::SLOW_BLINK)
    } else {
        Style::default()
    };
    let logged_user_content = Paragraph::new(Line::from(vec![
            Span::raw(format!("{}  @{}  ", logged_user.display_name, logged_user.username)),
            Span::styled("●", online_status_style),
        ]))
        .block(main_block.clone())
        .alignment(Alignment::Center);

    let screen = app.get_screen();
    let navbar_content = Paragraph::new(navbar(&screen))
        .block(main_block)
        .alignment(Alignment::Right);

    frame.render_widget(title_content, area);
    frame.render_widget(logged_user_content, area);
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
