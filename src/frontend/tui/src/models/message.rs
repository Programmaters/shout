use chrono::{DateTime, Utc};
use crate::models::Id;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub id: Id,
    pub sender: Id,
    pub datetime: DateTime<Utc>,
    pub content: String,
}