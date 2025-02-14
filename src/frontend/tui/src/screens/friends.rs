use crate::screens::ScreenSection;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FriendsScreen {
    pub section: FriendsSection,
    pub prev_section: FriendsSection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FriendsSection {
    Chats,
    Messages,
    Friends,
    Popup,
}

impl ScreenSection for FriendsSection {
    fn all() -> Vec<Self> {
        vec![Self::Chats, Self::Messages, Self::Friends]
    }
}
