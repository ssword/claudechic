use claudechic_core::models::ChatItem;
use ratatui::widgets::Widget;

pub struct ChatMessageWidget<'a> {
    pub message: &'a ChatItem,
}

impl<'a> ChatMessageWidget<'a> {
    pub fn new(message: &'a ChatItem) -> Self {
        Self { message }
    }
}

impl<'a> Widget for ChatMessageWidget<'a> {
    fn render(self, _area: ratatui::prelude::Rect, _buf: &mut ratatui::prelude::Buffer) {
        // Placeholder implementation
    }
}
