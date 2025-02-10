
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Profile,
    Chat,
    Friends,
}

impl Screen {
    pub fn next(self) -> Self {
        match self {
            Screen::Profile => Screen::Chat,
            Screen::Chat => Screen::Friends,
            Screen::Friends => Screen::Friends,
        }
    }

    pub fn previous(self) -> Self {
        match self {
            Screen::Friends => Screen::Chat,
            Screen::Chat => Screen::Profile,
            Screen::Profile => Screen::Profile
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            Screen::Profile,
            Screen::Chat,
            Screen::Friends,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Screen::Profile => "Profile",
            Screen::Chat => "Chat",
            Screen::Friends => "Friends",
        }
    }
}
