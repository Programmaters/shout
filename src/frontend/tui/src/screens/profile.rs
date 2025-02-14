use crate::screens::ScreenSection;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileScreen {
    pub section: ProfileSection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileSection {
    Profile,
    Logout,
}

impl ScreenSection for ProfileSection {
    fn all() -> Vec<Self> {
        vec![Self::Profile, Self::Logout]
    }
}
