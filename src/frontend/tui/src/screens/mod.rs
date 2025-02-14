pub mod chat;
pub mod friends;
pub mod profile;

use crate::screens::chat::{ChatScreen, ChatSection};
use crate::screens::friends::{FriendsScreen, FriendsSection};
use crate::screens::profile::{ProfileScreen, ProfileSection};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Screen {
    Profile(ProfileScreen),
    Chat(ChatScreen),
    Friends(FriendsScreen),
}

impl Screen {
    fn change_section(&mut self, next: bool) {
        if self.in_popup() {
            return; // dont change section if in popup
        }
        match self {
            Screen::Chat(chat) => {
                chat.section = if next {
                    chat.section.next()
                } else {
                    chat.section.prev()
                };
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

    pub fn toggle_popup(&mut self) {
        match self {
            Screen::Profile(profile) => {
                toggle_popup_state(
                    &mut profile.section,
                    &mut profile.prev_section,
                    ProfileSection::Popup,
                );
            }
            Screen::Chat(chat) => {
                toggle_popup_state(
                    &mut chat.section,
                    &mut chat.prev_section,
                    ChatSection::Popup,
                );
            }
            Screen::Friends(friends) => {
                toggle_popup_state(
                    &mut friends.section,
                    &mut friends.prev_section,
                    FriendsSection::Popup,
                );
            }
        }
    }

    pub fn in_popup(&self) -> bool {
        match self {
            Screen::Profile(profile) => profile.section == ProfileSection::Popup,
            Screen::Chat(chat) => chat.section == ChatSection::Popup,
            Screen::Friends(friends) => friends.section == FriendsSection::Popup,
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            Screen::Profile(ProfileScreen {
                section: ProfileSection::Settings,
                prev_section: ProfileSection::Settings,
            }),
            Screen::Chat(ChatScreen {
                section: ChatSection::Messages,
                prev_section: ChatSection::Messages,
                channels_index: None,
                members_index: None,
                input_field: "".to_string(),
                scroll_offset: 0,
                channel_selected: None,
                channels: vec![],
            }),
            Screen::Friends(FriendsScreen {
                section: FriendsSection::Messages,
                prev_section: FriendsSection::Messages,
            }),
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

pub trait ScreenSection: Copy + Sized {
    fn all() -> Vec<Self>;

    fn from_index(index: usize) -> Self {
        let all = Self::all();
        all.get(index)
            .copied()
            .unwrap_or_else(|| *all.last().unwrap())
    }

    fn to_index(self) -> usize
    where
        Self: PartialEq,
    {
        Self::all().into_iter().position(|s| s == self).unwrap()
    }

    fn next(&self) -> Self
    where
        Self: PartialEq,
    {
        let index = self.to_index();
        Self::from_index(index + 1)
    }

    fn prev(&self) -> Self
    where
        Self: PartialEq,
    {
        let index = self.to_index();
        Self::from_index(index.saturating_sub(1))
    }
}

fn toggle_popup_state<T: PartialEq + Copy>(section: &mut T, prev_section: &mut T, popup: T) {
    if *section != popup {
        // other => popup
        *prev_section = *section;
        *section = popup;
    } else {
        // popup => other
        *section = *prev_section;
    }
}
