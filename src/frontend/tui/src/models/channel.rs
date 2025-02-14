use crate::models::message::Message;
use crate::models::user::User;
use crate::models::Id;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Channel {
    pub id: Id,
    pub name: String,
    pub messages: Vec<Message>,
    pub members: Vec<User>,
}
