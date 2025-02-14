use ratatui::style::Color;

pub enum SelectState {
    Selected,
    Unselected,
}

impl SelectState {
    pub fn to_color(&self) -> Color {
        match self {
            SelectState::Selected => Color::White,
            SelectState::Unselected => Color::Gray,
        }
    }

    pub fn from_bool(condition: bool) -> Self {
        if condition {
            SelectState::Selected
        } else {
            SelectState::Unselected
        }
    }
}
