use chrono::Utc;
use crate::models::channel::Channel;
use crate::models::Id;
use crate::models::message::Message;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Screen {
    Profile(ProfileScreen),
    Chat(ChatScreen),
    Friends(FriendsScreen),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileScreen {
    pub section: ProfileSection,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatScreen {
    pub section: ChatSection,
    pub channels_index: Option<usize>,
    pub members_index: Option<usize>,
    pub input_field: String,
    pub scroll_offset: usize,
    pub channel_selected: Id,
    pub channels: Vec<Channel>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FriendsScreen {
    pub section: FriendsSection,
}

impl Screen {

    fn change_section(&mut self, next: bool) {
        match self {
            Screen::Chat(chat) => {
                chat.section = if next { chat.section.next() } else { chat.section.prev() };
                chat.channels_index = if chat.section == ChatSection::Channels {
                    Some(chat.channels_index.unwrap_or(0))
                } else {
                    None
                };
                chat.members_index = if chat.section == ChatSection::Members {
                    Some(chat.members_index.unwrap_or(0))
                } else {
                    None
                };
            }
            _ => {}
        }
    }

    pub fn prev_section(&mut self) {
        self.change_section(false);
    }

    pub fn next_section(&mut self) {
        self.change_section(true);
    }

    pub fn all() -> Vec<Self> {
        vec![
            Screen::Profile(ProfileScreen { section: ProfileSection::Profile }),
            Screen::Chat(ChatScreen {
                section: ChatSection::Messages,
                channels_index: None,
                members_index: None,
                input_field: "".to_string(),
                scroll_offset: 0,
                channel_selected: "".to_string(),
                channels: vec![],
            }),
            Screen::Friends(FriendsScreen { section: FriendsSection::Friends }),
        ]
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Screen::Profile(_) => "Profile",
            Screen::Chat(_) => "Chat",
            Screen::Friends(_) => "Friends",
        }
    }
}

impl ChatScreen {
    pub fn send_message(&mut self, sender: Id) {
        if self.input_field.is_empty() {
            return;
        }
        let message = Message {
            id: "".to_string(),
            sender,
            datetime: Utc::now(),
            content: self.input_field.clone(),
        };
        self.get_channel_mut().messages.push(message);
        self.input_field = "".to_string();
        self.scroll_offset = 0;
    }

    pub fn get_channel(&self) -> &Channel {
        let channel_id = &self.channel_selected;
        self.channels.iter().find(|c| &c.id == channel_id).expect("channel not found")
    }

    pub fn get_channel_mut(&mut self) -> &mut Channel {
        let channel_id = &self.channel_selected;
        self.channels.iter_mut().find(|c| &c.id == channel_id).expect("channel not found")
    }

    pub fn select_channel(&mut self) {
        if let Some(index) = self.channels_index {
            self.channel_selected = self.channels[index].id.clone();
            self.members_index = None;
        }
    }
}


pub trait ScreenSection: Copy + Sized {
    fn all() -> Vec<Self>;

    fn from_index(index: usize) -> Self {
        let all = Self::all();
        all.get(index).copied().unwrap_or_else(|| *all.last().unwrap())
    }

    fn to_index(self) -> usize where Self: PartialEq {
        Self::all().into_iter().position(|s| s == self).unwrap()
    }

    fn next(&self) -> Self where Self: PartialEq {
        let index = self.to_index();
        Self::from_index(index + 1)
    }

    fn prev(&self) -> Self where Self: PartialEq {
        let index = self.to_index();
        Self::from_index(index.saturating_sub(1))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChatSection {
    Channels,
    Messages,
    Members,
}

impl ScreenSection for ChatSection {
    fn all() -> Vec<Self> {
        vec![Self::Channels, Self::Messages, Self::Members]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileSection {
    Profile,
    Logout
}

impl ScreenSection for ProfileSection {
    fn all() -> Vec<Self> {
        vec![Self::Profile, Self::Logout]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FriendsSection {
    Friends,
    Messages,
    Requests
}

impl ScreenSection for FriendsSection {
    fn all() -> Vec<Self> {
        vec![Self::Friends, Self::Messages, Self::Requests]
    }
}