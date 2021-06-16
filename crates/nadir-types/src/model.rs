use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A group of messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "snake_case")]
pub struct MessageGroup {
    /// A unique identifier for this group.
    pub id: String,

    /// Importance parameter, groups with higher importance will show in the
    /// front.
    ///
    /// Defaults to 0.
    pub importance: i32,

    /// The title of this group.
    pub title: String,

    /// A message capacity hint for this group.
    ///
    /// This message group will display at most this many messages, but
    /// implementations may choose smaller numbers when screen space is not
    /// enough.
    ///
    /// Defaults to 10.
    pub capacity: u32,

    /// A message capacity hint for pinned messages. The semantics is similar to
    /// [`capacity`].
    ///
    /// Defaults to 5.
    pub pinned_capacity: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct Message {
    /// The identifier of this message. Messages with the same ID and the same
    /// [`MessageGroup`] will replace each other upon receiving.
    ///
    /// Example: group name in instant messaging tool, email address in maildir.
    pub id: String,

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

impl Default for MessageGroup {
    fn default() -> Self {
        MessageGroup {
            id: Default::default(),
            importance: 0,
            title: Default::default(),
            capacity: 10,
            pinned_capacity: 5,
        }
    }
}
