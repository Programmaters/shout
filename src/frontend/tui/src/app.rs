use crate::models::channel::Channel;
use crate::models::message::Message;
use crate::models::screen::Screen;
use crate::models::server::Server;
use crate::models::user::User;
use crate::ui::render_ui;
use chrono::Utc;
use color_eyre::eyre::WrapErr;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{DefaultTerminal, Frame};

pub struct App {
    pub running: bool,
    pub screen: Screen,
    pub input_box: String,
    pub scroll_offset: usize,
    pub logged_user: Option<User>,
    pub server_selected: Option<Server>,
    pub channel_selected: Option<Channel>,
}

impl App {
    pub fn new() -> Self {
        App {
            running: true,
            screen: Screen::Chat,
            input_box: "".to_string(),
            scroll_offset: 0,
            logged_user: Some(User {
                id: "123".to_string(),
                username: "me".to_string(),
                display_name: "Me".to_string(),
            }),
            server_selected: Some(Server {
                id: "234".to_string(),
                name: "Server".to_string(),
                channels: vec!["345".to_string()],
                members: vec!["123".to_string(), "321".to_string()],
            }),
            channel_selected: Some(Channel {
                id: "345".to_string(),
                name: "chat".to_string(),
                messages: vec![],
            }),
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
                _ => {}
            }
        } else {
            match e.code {
                KeyCode::Esc => self.quit(),
                KeyCode::Up => {
                    // older messages
                    self.scroll_offset = self.scroll_offset.saturating_add(1)
                },
                KeyCode::Down => {
                    // more recent messages
                    self.scroll_offset = self.scroll_offset.saturating_sub(1)
                },
                char => self.handle_input_box(char),
            }
        }
        Ok(())
    }

    fn handle_input_box(&mut self, e: KeyCode) {
        match e {
            KeyCode::Enter => {
                if self.input_box.is_empty() {
                    return;
                }
                let message = Message {
                    id: "".to_string(),
                    sender: "me".to_string(),
                    datetime: Utc::now(),
                    content: self.input_box.clone(),
                };
                if let Some(channel) = self.channel_selected.as_mut() {
                    channel.messages.push(message);
                }
                self.input_box = "".to_string();
                self.scroll_offset = 0; // scroll to most recent message
            }
            KeyCode::Backspace => {
                self.input_box.pop();
            }
            KeyCode::Char(c) => {
                self.input_box.push(c);
            },
            _ => {}
        };
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