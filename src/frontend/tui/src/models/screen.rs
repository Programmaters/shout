#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Profile(ProfileSection),
    Chat(ChatSection),
    Friends(FriendsSection),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileSection {
    Profile,
    Logout
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChatSection {
    Channels,
    Messages,
    Members
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FriendsSection {
    Friends,
    Messages,
    Requests
}

impl Screen {
    pub fn next(self) -> Self {
        match self {
            Screen::Profile(_) => Screen::Chat(ChatSection::Messages),
            Screen::Chat(_) => Screen::Friends(FriendsSection::Messages),
            Screen::Friends(_) => Screen::Friends(FriendsSection::Messages),
        }
    }

    pub fn previous(self) -> Self {
        match self {
            Screen::Friends(_) => Screen::Chat(ChatSection::Messages),
            Screen::Chat(_) => Screen::Profile(ProfileSection::Profile),
            Screen::Profile(_) => Screen::Profile(ProfileSection::Profile)
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            Screen::Profile(ProfileSection::Profile),
            Screen::Chat(ChatSection::Messages),
            Screen::Friends(FriendsSection::Messages),
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Screen::Profile(_) => "Profile",
            Screen::Chat(_) => "Chat",
            Screen::Friends(_) => "Friends",
        }
    }
}

impl ChatSection {
    pub fn next(self) -> Self {
        match self {
            ChatSection::Channels => ChatSection::Messages,
            ChatSection::Messages => ChatSection::Members,
            _ => self
        }
    }

    pub fn prev(self) -> Self {
        match self {
            ChatSection::Members => ChatSection::Messages,
            ChatSection::Messages => ChatSection::Channels,
            _ => self
        }
    }
}