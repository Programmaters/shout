use crate::api::Api;
use crate::events::handle_events;
use crate::models::user::User;
use crate::models::Id;
use crate::screens::chat::{ChatScreen, ChatSection};
use crate::screens::friends::{FriendsScreen, FriendsSection};
use crate::screens::profile::{ProfileScreen, ProfileSection};
use crate::screens::Screen;
use crate::ui::render_ui;
use color_eyre::Result;
use ratatui::{DefaultTerminal, Frame};

pub struct App {
    pub api: Api,
    pub running: bool,
    pub screen_index: usize,
    pub screens: Vec<Screen>,
    pub logged_user: Option<User>,
    pub users: Vec<User>,
}

impl App {
    pub fn new() -> Self {
        App {
            api: Api::new(),
            running: true,
            screen_index: 1,
            screens: Screen::all(),
            logged_user: None,
            users: vec![],
        }
    }

    fn draw(&self, frame: &mut Frame) {
        render_ui(self, frame);
    }

    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        let logged_user = self.api.login().await.unwrap();
        let users = self.api.get_users().await.unwrap();
        let channels = self.api.get_channels().await.unwrap();
        if let Some(Screen::Chat(ref mut chat)) = self.screens.get_mut(1) {
            self.logged_user = logged_user.clone();
            self.users = users;
            chat.channels = channels.clone();
            chat.channel_selected = channels.first().map(|c| c.id.clone());
        }

        while self.running {
            terminal.draw(|frame| self.draw(frame))?;
            handle_events(self)?;
        }
        Ok(())
    }

    pub fn get_user(&self, id: Id) -> User {
        self.users
            .iter()
            .find(|member| member.id == id)
            .unwrap()
            .clone()
    }

    pub fn get_screen_mut(&mut self) -> &mut Screen {
        &mut self.screens[self.screen_index]
    }

    pub fn get_screen(&self) -> Screen {
        self.screens[self.screen_index].clone()
    }

    pub fn prev_screen(&mut self) {
        if self.screen_index > 0 {
            self.screen_index -= 1;
        }
    }

    pub fn next_screen(&mut self) {
        if self.screen_index < self.screens.len() - 1 {
            self.screen_index += 1;
        }
    }

    pub fn prev_section(&mut self) {
        let curr_screen = &mut self.screens[self.screen_index];
        curr_screen.prev_section();
    }

    pub fn next_section(&mut self) {
        let curr_screen = &mut self.screens[self.screen_index];
        curr_screen.next_section();
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}
