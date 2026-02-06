use ratatui::style::Color;

pub struct Theme {
    pub user_message: Color,
    pub assistant_message: Color,
    pub tool_use: Color,
    pub error: Color,
    pub success: Color,
    pub warning: Color,
}

impl Theme {
    pub fn default() -> Self {
        Self {
            user_message: Color::Rgb(204, 119, 0),
            assistant_message: Color::Rgb(51, 68, 85),
            tool_use: Color::Rgb(51, 51, 51),
            error: Color::Red,
            success: Color::Green,
            warning: Color::Yellow,
        }
    }
}
