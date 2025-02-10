use chrono::{DateTime, Utc};
use crate::models::Id;

pub struct Message {
    pub id: Id,
    pub sender: Id,
    pub datetime: DateTime<Utc>,
    pub content: String,
}