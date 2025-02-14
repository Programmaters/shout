use crate::app::App;
use crate::screens::chat::{ChatScreen, ChatSection};
use crate::screens::Screen;
use crossterm::event::KeyCode;

pub fn handle_navigation(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Right => app.next_section(),
        KeyCode::Left => app.prev_section(),
        _ => match app.get_screen_mut() {
            Screen::Chat(ref mut chat) => handle_chat_navigation(key, chat),
            _ => {} // todo
        },
    }
}

fn handle_chat_navigation(key: KeyCode, chat: &mut ChatScreen) {
    match chat.section {
        ChatSection::Messages => {
            match key {
                KeyCode::Up => chat.scroll_offset = chat.scroll_offset.saturating_add(1), // older messages
                KeyCode::Down => chat.scroll_offset = chat.scroll_offset.saturating_sub(1), // newer messages
                _ => {}
            }
        }
        ChatSection::Channels => match key {
            KeyCode::Up => {
                if let Some(index) = chat.channels_index {
                    chat.channels_index = Some(index.saturating_sub(1));
                }
            }
            KeyCode::Down => {
                if let Some(index) = chat.channels_index {
                    if index < chat.channels.len() - 1 {
                        chat.channels_index = Some(index.saturating_add(1));
                    }
                } else {
                    chat.channels_index = Some(0);
                }
            }
            _ => {}
        },
        ChatSection::Members => match key {
            KeyCode::Up => {
                if let Some(index) = chat.members_index {
                    chat.members_index = Some(index.saturating_sub(1));
                }
            }
            KeyCode::Down => {
                if let Some(index) = chat.members_index {
                    let channel = chat.get_channel();
                    if index < channel.members.len() - 1 {
                        chat.members_index = Some(index.saturating_add(1));
                    }
                } else {
                    chat.members_index = Some(0);
                }
            }
            _ => {}
        },
        _ => {}
    }
}
