use crate::models::channel::Channel;
use crate::models::message::Message;
use crate::models::Id;
use crate::screens::ScreenSection;
use chrono::Utc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatScreen {
    pub section: ChatSection,
    pub channels_index: Option<usize>,
    pub members_index: Option<usize>,
    pub input_field: String,
    pub scroll_offset: usize,
    pub channel_selected: Id,
    pub channels: Vec<Channel>,
}

impl ChatScreen {
    pub fn create_message(&mut self, sender: Id) -> Option<Message> {
        if self.input_field.is_empty() {
            return None;
        }
        let message = Message {
            id: "".to_string(),
            sender,
            datetime: Utc::now(),
            content: self.input_field.clone(),
        };

        self.get_channel_mut().messages.push(message.clone());
        self.input_field = "".to_string();
        self.scroll_offset = 0;
        Some(message)
    }

    pub fn get_channel(&self) -> &Channel {
        let channel_id = &self.channel_selected;
        self.channels
            .iter()
            .find(|c| &c.id == channel_id)
            .expect("channel not found")
    }

    pub fn get_channel_mut(&mut self) -> &mut Channel {
        let channel_id = &self.channel_selected;
        self.channels
            .iter_mut()
            .find(|c| &c.id == channel_id)
            .expect("channel not found")
    }

    pub fn select_channel(&mut self) {
        if let Some(index) = self.channels_index {
            self.channel_selected = self.channels[index].id.clone();
            self.members_index = None;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChatSection {
    Channels,
    Messages,
    Members,
}

impl ScreenSection for ChatSection {
    fn all() -> Vec<Self> {
        vec![Self::Channels, Self::Messages, Self::Members]
    }
}
