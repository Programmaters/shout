use crate::models::Id;
use crate::models::message::Message;
use crate::models::user::User;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Channel {
    pub id: Id,
    pub name: String,
    pub messages: Vec<Message>,
    pub members: Vec<User>,
}
