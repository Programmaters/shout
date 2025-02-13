use crate::events::handle_events;
use crate::models::channel::Channel;
use crate::models::screen::{ChatScreen, ChatSection, FriendsScreen, FriendsSection, ProfileScreen, ProfileSection, Screen};
use crate::models::user::User;
use crate::models::Id;
use crate::ui::render_ui;
use color_eyre::Result;
use ratatui::{DefaultTerminal, Frame};

pub struct App {
    pub running: bool,
    pub screen_index: usize,
    pub screens: Vec<Screen>,
    pub logged_user: Option<User>,
    pub users: Vec<User>,
}

impl App {
    pub fn new() -> Self {
        let (user1, user2) = create_sample_users();
        let (channel1, channel2) = create_sample_channels(&user1, &user2);
        let chat_screen = Screen::Chat(
            ChatScreen {
                section: ChatSection::Messages,
                channels_index: None,
                members_index: None,
                input_field: "".to_string(),
                scroll_offset: 0,
                channel_selected: channel1.clone().id,
                channels: vec![channel1, channel2],
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
            running: true,
            screen_index: 1,
            screens: vec![profile_screen, chat_screen, friends_screen],
            logged_user: Some(user1.clone()),
            users: vec![user1, user2],
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
        let mut curr_screen = &mut self.screens[self.screen_index];
        curr_screen.prev_section();
    }

    pub fn next_section(&mut self) {
        let mut curr_screen = &mut self.screens[self.screen_index];
        curr_screen.next_section();
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}


fn create_sample_users() -> (User, User) {
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
    (user1, user2)
}

fn create_sample_channels(user1: &User, user2: &User) -> (Channel, Channel){
    let channel1 = Channel {
        id: "345".to_string(),
        name: "chat".to_string(),
        members: vec![user1.clone(), user2.clone()],
        messages: vec![],
    };
    let channel2 = Channel {
        id: "567".to_string(),
        name: "other-chat".to_string(),
        members: vec![user1.clone()],
        messages: vec![],
    };
    (channel1, channel2)
}