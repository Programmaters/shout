use crate::models::channel::Channel;
use crate::models::message::Message;
use crate::models::screen::{ChatSection, Screen};
use crate::models::user::User;
use crate::ui::render_ui;
use chrono::Utc;
use color_eyre::eyre::WrapErr;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{DefaultTerminal, Frame};
use crate::models::Id;

pub struct App {
    pub running: bool,
    pub screen: Screen,
    pub input_field: String,
    pub scroll_offset: usize,
    pub channels: Vec<Channel>,
    pub logged_user: Option<User>,
    pub channel_selected: Id,
    pub channel_index: Option<usize>,
    pub members_index: Option<usize>,
}

impl App {
    pub fn new() -> Self {
        let user1 = User {
            id: "123".to_string(),
            username: "user1".to_string(),
            display_name: "User 1".to_string(),
            online: true,
        };
        let user2 = User {
            id: "321".to_string(),
            username: "user2".to_string(),
            display_name: "User 2".to_string(),
            online: false,
        };
        let channel = Channel {
            id: "345".to_string(),
            name: "chat".to_string(),
            members: vec![user1.clone(), user2],
            messages: vec![],
        };
        let other_channel = Channel {
            id: "567".to_string(),
            name: "other-chat".to_string(),
            members: vec![user1.clone()],
            messages: vec![],
        };
        App {
            running: true,
            screen: Screen::Chat(ChatSection::Messages),
            input_field: "".to_string(),
            scroll_offset: 0,
            channels: vec![channel.clone(), other_channel],
            logged_user: Some(user1),
            channel_selected: channel.id,
            channel_index: Some(0),
            members_index: None,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while self.running {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events().wrap_err("failed to handle events")?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        render_ui(self, frame);
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(e) if e.kind == KeyEventKind::Press => {
                self.handle_key_event(e)
                    .wrap_err_with(|| format!("failed to handle key event:\n{e:#?}"))
            }
            _ => Ok(())
        }
    }

    fn handle_key_event(&mut self, e: KeyEvent) -> Result<()> {
        if e.modifiers.contains(KeyModifiers::CONTROL) {
            match e.code {
                KeyCode::Left => self.prev_screen(),
                KeyCode::Right => self.next_screen(),
                KeyCode::Backspace if matches!(self.screen, Screen::Chat(ChatSection::Messages)) => self.field_ctrl_backspace(),
                _ => {}
            }
        } else {
            match e.code {
                KeyCode::Esc => self.quit(),
                KeyCode::Up | KeyCode::Down | KeyCode::Right | KeyCode::Left => self.handle_directional_key_event(e.code),
                char => self.handle_char_input(char),
            }
        }
        Ok(())
    }

    fn handle_directional_key_event(&mut self, key: KeyCode) {
        match self.screen {
            Screen::Chat(section) => {
                match key {
                    KeyCode::Right => self.screen = Screen::Chat(section.next()),
                    KeyCode::Left => self.screen = Screen::Chat(section.prev()),
                    _ => {}
                };

                match section {
                    ChatSection::Messages => {
                        match key {
                            KeyCode::Up => self.scroll_offset = self.scroll_offset.saturating_add(1), // older messages
                            KeyCode::Down => self.scroll_offset = self.scroll_offset.saturating_sub(1), // newer messages
                            _ => {}
                        }
                    }
                    ChatSection::Channels => {
                        match key {
                            KeyCode::Up => {
                                if let Some(index) = self.channel_index {
                                    self.channel_index = Some(index.saturating_sub(1));
                                }

                            },
                            KeyCode::Down => {
                                if let Some(index) = self.channel_index {
                                    if index < self.channels.len() - 1 {
                                        self.channel_index = Some(index.saturating_add(1));
                                    }
                                } else {
                                    self.channel_index = Some(0);
                                }

                            }
                            _ => {}
                        }
                    }
                    ChatSection::Members => {
                        match key {
                            KeyCode::Up => {
                                if let Some(index) = self.members_index {
                                    self.members_index = Some(index.saturating_sub(1));
                                }
                            },
                            KeyCode::Down => {
                                if let Some(index) = self.members_index {
                                    let channel = self.channels[index].clone();
                                    if index < channel.members.len() - 1 {
                                        self.members_index = Some(index.saturating_add(1));
                                    }
                                } else {
                                    self.members_index = Some(0);
                                }

                            }
                            _ => {}
                        }
                    }
                }

            }
            _ => {}
        }
    }

    fn handle_char_input(&mut self, key: KeyCode) {
        match self.screen {
            Screen::Chat(section) => {
                match section {
                    ChatSection::Messages => self.handle_chat_input(key),
                    ChatSection::Channels => self.handle_channels_input(key),
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn handle_chat_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Enter => {
                if self.input_field.is_empty() {
                    return;
                }
                let message = Message {
                    id: "".to_string(),
                    sender: self.logged_user.clone().unwrap().id,
                    datetime: Utc::now(),
                    content: self.input_field.clone(),
                };
                self.get_channel_mut().messages.push(message);
                self.input_field = "".to_string();
                self.scroll_offset = 0; // scroll to most recent message
            }
            KeyCode::Backspace => {
                self.input_field.pop();
            }
            KeyCode::Char(c) => {
                self.input_field.push(c);
            },
            _ => {}
        };
    }

    fn handle_channels_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Enter => {
                if let Some(index) = self.channel_index {
                    self.channel_selected = self.channels[index].clone().id;
                    self.members_index = None;
                }
            }
            _ => {}
        }
    }

    pub fn get_channel(&self) -> Channel {
        let channel_id = self.channel_selected.clone();
        self.channels.iter().find(|c| c.id == channel_id).unwrap().clone()
    }

    fn get_channel_mut(&mut self) -> &mut Channel {
        let channel_id = self.channel_selected.clone();
        self.channels.iter_mut().find(|c| c.id == channel_id).unwrap()
    }

    fn field_ctrl_backspace(&mut self) {
        let parts: Vec<&str> = self.input_field.trim().split(' ').collect();
        self.input_field = parts[..parts.len().saturating_sub(1)].join(" ");
    }

    fn prev_screen(&mut self) {
        self.screen = self.screen.previous();
    }

    fn next_screen(&mut self) {
        self.screen = self.screen.next();
    }

    fn quit(&mut self) {
        self.running = false;
    }
}