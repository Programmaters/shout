use crate::models::Id;

pub struct Server {
    pub id: Id,
    pub name: String,
    pub channels: Vec<Id>,
    pub members: Vec<Id>,
}