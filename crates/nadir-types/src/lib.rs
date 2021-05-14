use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    Add(AddMsg),
    Remove(RemoveMsg),
    AddNamespace(AddNamespaceMsg),
    Config,
}

/// Add or replace a namespace in Nadir
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddNamespaceMsg {
    /// The namespace's name
    pub ns: Vec<String>,
    #[serde(default)]
    pub init_cnt: usize,
}

/// Add notifications in Nadir
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddMsg {
    /// The namespace of notifications
    pub ns: Vec<String>,
    /// The notifications to add
    pub items: Vec<Notification>,
}

/// Remove notifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveMsg {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    /// The identifier of this notification, must be unique in namespace
    pub id: String,
    /// Additional tags of this notification
    #[serde(default)]
    pub tags: Vec<String>,
    /// The title of this notification
    pub title: String,
    /// The send time of this notification
    pub time: Option<DateTime<Utc>>,
}
