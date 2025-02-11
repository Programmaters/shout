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
    ChannelList,
    Chat,
    MemberList
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FriendsSection {
    FriendList,
    Chat,
    FriendRequests
}

impl Screen {
    pub fn next(self) -> Self {
        match self {
            Screen::Profile(_) => Screen::Chat(ChatSection::Chat),
            Screen::Chat(_) => Screen::Friends(FriendsSection::Chat),
            Screen::Friends(_) => Screen::Friends(FriendsSection::Chat),
        }
    }

    pub fn previous(self) -> Self {
        match self {
            Screen::Friends(_) => Screen::Chat(ChatSection::Chat),
            Screen::Chat(_) => Screen::Profile(ProfileSection::Profile),
            Screen::Profile(_) => Screen::Profile(ProfileSection::Profile)
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            Screen::Profile(ProfileSection::Profile),
            Screen::Chat(ChatSection::Chat),
            Screen::Friends(FriendsSection::Chat),
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
            ChatSection::ChannelList => ChatSection::Chat,
            ChatSection::Chat => ChatSection::MemberList,
            _ => self
        }
    }

    pub fn prev(self) -> Self {
        match self {
            ChatSection::MemberList => ChatSection::Chat,
            ChatSection::Chat => ChatSection::ChannelList,
            _ => self
        }
    }
}