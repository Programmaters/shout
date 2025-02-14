mod navigation;

use crate::app::App;
use crate::events::navigation::handle_navigation;
use crate::screens::chat::ChatSection;
use crate::screens::Screen;
use color_eyre::Result;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

pub fn handle_events(app: &mut App) -> Result<()> {
    match event::read()? {
        Event::Key(e) if e.kind == KeyEventKind::Press => handle_key_event(app, e),
        _ => {}
    }
    Ok(())
}

fn handle_key_event(app: &mut App, e: KeyEvent) {
    if e.modifiers.contains(KeyModifiers::CONTROL) {
        handle_control_key(app, e)
    } else {
        handle_normal_key(app, e)
    }
}

fn handle_control_key(app: &mut App, e: KeyEvent) {
    match e.code {
        KeyCode::Left => app.prev_screen(),
        KeyCode::Right => app.next_screen(),
        KeyCode::Backspace => handle_ctrl_backspace(app),
        _ => {}
    }
}

fn handle_normal_key(app: &mut App, e: KeyEvent) {
    match e.code {
        KeyCode::Esc => app.quit(),
        KeyCode::Up | KeyCode::Down | KeyCode::Right | KeyCode::Left => {
            handle_navigation(app, e.code)
        }
        KeyCode::Tab => handle_popup_state(app),
        char => handle_char(app, char),
    }
}

fn handle_char(app: &mut App, key: KeyCode) {
    let logged_user_id = app.logged_user.clone().unwrap().id;
    match app.get_screen_mut() {
        Screen::Chat(ref mut chat) => match chat.section {
            ChatSection::Messages => match key {
                KeyCode::Enter => {
                    if let Some(msg) = chat.get_message(logged_user_id) {
                        let channel_selected = chat.channel_selected.clone().unwrap();
                        let api = app.api.clone();
                        tokio::spawn(async move { api.send_message(msg, channel_selected).await });
                    }
                }
                KeyCode::Backspace => {
                    chat.input_field.pop();
                }
                KeyCode::Char(c) => {
                    chat.input_field.push(c);
                }
                _ => {}
            },
            ChatSection::Channels => chat.select_channel(),
            _ => {}
        },
        _ => {}
    }
}

fn handle_popup_state(app: &mut App) {
    app.get_screen_mut().toggle_popup();
}

fn handle_ctrl_backspace(app: &mut App) {
    if let Screen::Chat(ref mut chat) = app.get_screen_mut() {
        // delete last word from field
        let parts: Vec<&str> = chat.input_field.trim().split(' ').collect();
        chat.input_field = parts[..parts.len().saturating_sub(1)].join(" ");
    }
}
