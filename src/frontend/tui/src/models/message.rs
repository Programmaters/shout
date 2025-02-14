use crate::models::Id;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub id: Id,
    pub sender: Id,
    pub datetime: DateTime<Utc>,
    pub content: String,
}
