use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

/// A group of messages.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MessageGroup {
    /// A unique identifier for this group.
    pub id: SmolStr,

    /// The title of this group.
    pub title: String,

    /// A message capacity hint for this group.
    ///
    /// Message sent to this group will
    /// be discarded in LRU order when the total amount is above this capacity.
    /// The notify server is free to choose any number smaller than this capacity.
    pub capacity: usize,

    /// A message capacity hint for pinned messages. See [`capacity`].
    pub pinned_capacity: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Message {
    /// The identifier of this message. Messages with the same ID and the same
    /// [`MessageGroup`] will replace each other upon receiving.
    ///
    /// Example: group name in instant messaging tool, email address in maildir.
    pub id: SmolStr,

    /// A counter showing how many "real" notification is currently
    /// covered by this message.
    ///
    /// Example: message count of instant message chats.
    pub counter: Option<u64>,

    /// Tag sections of this message
    #[serde(default)]
    pub tags: Vec<String>,

    /// The body section of this message
    #[serde(default)]
    pub body: String,

    /// The send time of this message
    #[serde(default)]
    pub time: Option<DateTime<Utc>>,
}
