use crate::screens::ScreenSection;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileScreen {
    pub section: ProfileSection,
    pub prev_section: ProfileSection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileSection {
    Settings,
    Info,
    Popup,
}

impl ScreenSection for ProfileSection {
    fn all() -> Vec<Self> {
        vec![Self::Settings, Self::Info]
    }
}
