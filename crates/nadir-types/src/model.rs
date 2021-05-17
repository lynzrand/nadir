use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageGroup {
    pub id: SmolStr,
    pub title: String,
    pub capacity: usize,
    pub pinned_capacity: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// The identifier of this notification, must be unique in namespace
    pub id: SmolStr,
    /// Tag sections of this notification
    #[serde(default)]
    pub tags: Vec<String>,
    /// The text section of this notification
    pub text: String,
    /// The send time of this notification
    pub time: Option<DateTime<Utc>>,
}
