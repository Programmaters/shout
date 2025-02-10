use ratatui::prelude::{Line, Stylize, Text};
use ratatui::widgets::{Block, Paragraph};
use crate::app::App;
use crate::models::screen::Screen;
use crate::ui::navbar::navbar;

pub fn main_content(app: &App) -> Paragraph {
    let navbar = navbar(app);
    let title = Line::from(" Shout ".bold());
    let main_block = Block::bordered()
        .title(title.left_aligned())
        .title(navbar.right_aligned());

    let content: Paragraph = match app.screen {
        Screen::Profile => profile_screen(app),
        Screen::Servers => servers_screen(app),
        Screen::Friends => friends_screen(app)
    };

    content.block(main_block).centered()
}

pub fn profile_screen(app: &App) -> Paragraph {
    let profile_text = Text::from("This is the profile screen");
    Paragraph::new(profile_text)
}

pub fn servers_screen(app: &App) -> Paragraph {
    let servers_text = Text::from("This is the servers screen");
    Paragraph::new(servers_text)
}

pub fn friends_screen(app: &App) -> Paragraph {
    let friends_text = Text::from("This is the friends screen");
    Paragraph::new(friends_text)
}