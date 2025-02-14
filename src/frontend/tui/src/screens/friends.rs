use crate::screens::ScreenSection;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FriendsScreen {
    pub section: FriendsSection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FriendsSection {
    Friends,
    Messages,
    Requests,
}

impl ScreenSection for FriendsSection {
    fn all() -> Vec<Self> {
        vec![Self::Friends, Self::Messages, Self::Requests]
    }
}
