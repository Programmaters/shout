use lazy_static::lazy_static;
use reqwest::Client;
use crate::models::channel::Channel;
use crate::models::Id;
use crate::models::message::Message;
use crate::models::user::User;

lazy_static! {
    static ref BASE_URL: &'static str = "http://localhost:8000";
    static ref USER1: User = User {
        id: "123".to_string(),
        username: "user1".to_string(),
        display_name: "User 1".to_string(),
        online: true,
    };
    static ref USER2: User = User {
        id: "321".to_string(),
        username: "user2".to_string(),
        display_name: "User 2".to_string(),
        online: false,
    };
    static ref CHANNEL1: Channel = Channel {
        id: "345".to_string(),
        name: "chat".to_string(),
        members: vec![USER1.clone(), USER2.clone()],
        messages: vec![],
    };
    static ref CHANNEL2: Channel = Channel {
        id: "567".to_string(),
        name: "other-chat".to_string(),
        members: vec![USER1.clone()],
        messages: vec![],
    };
}

#[derive(Clone)]
pub struct Api {
    client: Client,
    base_url: String,
}

// todo: make actual api calls
impl Api {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: BASE_URL.to_string(),
        }
    }

    pub async fn login(&self) -> Result<Option<User>, &'static str> {
        Ok(Some(USER1.clone()))
    }

    pub async fn get_users(&self) -> Result<Vec<User>, &'static str>  {
        Ok(vec![USER1.clone(), USER2.clone()])
    }

    pub async fn get_channels(&self) -> Result<Vec<Channel>, &'static str> {
        Ok(vec![CHANNEL1.clone(), CHANNEL2.clone()])
    }

    pub async fn send_message(&self, _message: Message, _channel: Id) -> Result<(), &'static str> {
        Ok(())
    }
}