
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Profile,
    Servers,
    Friends,
}

impl Screen {
    pub fn next(self) -> Self {
        match self {
            Screen::Profile => Screen::Servers,
            Screen::Servers => Screen::Friends,
            Screen::Friends => Screen::Friends,
        }
    }

    pub fn previous(self) -> Self {
        match self {
            Screen::Friends => Screen::Servers,
            Screen::Servers => Screen::Profile,
            Screen::Profile => Screen::Profile
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            Screen::Profile,
            Screen::Servers,
            Screen::Friends,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Screen::Profile => "Profile",
            Screen::Servers => "Servers",
            Screen::Friends => "Friends",
        }
    }
}
