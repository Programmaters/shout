use crate::models::Id;
use crate::models::message::Message;

pub struct Channel {
    pub id: Id,
    pub name: String,
    pub messages: Vec<Message>,
}
