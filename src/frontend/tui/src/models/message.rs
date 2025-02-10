use crate::models::Id;

pub struct Message {
    pub id: Id,
    pub sender: Id,
    pub timestamp: String,
    pub content: String,
}