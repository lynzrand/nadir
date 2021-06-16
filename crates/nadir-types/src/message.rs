use serde::{Deserialize, Serialize};

use crate::model::{Message, MessageGroup};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "_t")]
pub enum ApiMessage {
    Add(AddMsg),
    Remove(RemoveMsg),
    PutGroup(PutGroupMsg),
    SetGroupCounter(SetGroupCounterMsg),
    Config,
}

/// Add notifications in Nadir
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddMsg {
    /// The group ID
    pub group: String,

    /// The messages to add.
    ///
    /// Messages are added in reverse order, _i.e._ messages that appear later
    /// in this list will be added to the front.
    pub items: Vec<Message>,
}

/// Remove notifications
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoveMsg {
    /// The group ID
    pub group: String,

    /// The IDs of messages to remove
    pub items: Vec<String>,
}

/// Add or replace a namespace in Nadir
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PutGroupMsg {
    /// Group metadata
    pub group: MessageGroup,
}

/// Sets the counter field on a specific group.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetGroupCounterMsg {
    /// The group ID
    pub group: String,
    /// The counter ID
    pub counter: u64,
}
