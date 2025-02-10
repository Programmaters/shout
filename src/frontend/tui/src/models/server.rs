use crate::models::channel::Channel;
use crate::models::user::User;

pub struct Server {
    pub id: String,
    pub name: String,
    pub channels: Vec<Channel>,
    pub members: Vec<User>,
}