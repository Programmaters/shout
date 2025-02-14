use crate::events::handle_events;
use crate::models::screen::{ChatScreen, ChatSection, FriendsScreen, FriendsSection, ProfileScreen, ProfileSection, Screen};
use crate::models::user::User;
use crate::models::Id;
use crate::ui::render_ui;
use color_eyre::Result;
use ratatui::{DefaultTerminal, Frame};
use crate::api::Api;

pub struct App {
    pub api: Api,
    pub running: bool,
    pub screen_index: usize,
    pub screens: Vec<Screen>,
    pub logged_user: Option<User>,
    pub users: Vec<User>,
}

impl App {
    pub async fn new() -> Self {
        let api = Api::new();
        let logged_user = api.login().await.unwrap();
        let users = api.get_users().await.unwrap();
        let channels = api.get_channels().await.unwrap();
        let chat_screen = Screen::Chat(
            ChatScreen {
                section: ChatSection::Messages,
                channels_index: None,
                members_index: None,
                input_field: "".to_string(),
                scroll_offset: 0,
                channel_selected: channels[0].id.clone(),
                channels,
            }
        );

        let profile_screen = Screen::Profile(
            ProfileScreen {
                section: ProfileSection::Profile,
            }
        );
        let friends_screen = Screen::Friends(
            FriendsScreen {
                section: FriendsSection::Friends,
            }
        );
        App {
            api,
            running: true,
            screen_index: 1,
            screens: vec![profile_screen, chat_screen, friends_screen],
            logged_user,
            users,
        }
    }

    fn draw(&self, frame: &mut Frame) {
        render_ui(self, frame);
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while self.running {
            terminal.draw(|frame| self.draw(frame))?;
            handle_events(self)?;
        }
        Ok(())
    }

    pub fn get_user(&self, id: Id) -> User {
        self.users.iter().find(|member| member.id == id).unwrap().clone()
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