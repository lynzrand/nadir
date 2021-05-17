use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

use crate::model::{Message, MessageGroup};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiMessage {
    Add(AddMsg),
    Remove(RemoveMsg),
    PutGroup(PutGroupMsg),
    Config,
}

/// Add or replace a namespace in Nadir
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutGroupMsg {
    /// Group metadata
    pub meta: MessageGroup,

    /// Initial message count for group
    #[serde(default)]
    pub init_cnt: usize,
}

/// Add notifications in Nadir
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddMsg {
    /// The namespace of notifications
    pub group: String,
    /// The notifications to add
    pub items: Vec<Message>,
}

/// Remove notifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveMsg {}
