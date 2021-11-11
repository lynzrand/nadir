use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HandshakeServerMessage {
    server_name: String,
    server_version: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HandshakeClientMessage {
    client_name: String,
    client_version: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ClientMessage {
    /// Create or replace a notify group.
    PutGroup(NotifyGroup),
    /// Create or replace a notify within a group.
    PutNotify(Notify),
    /// Remove a notify group.
    RemoveGroup { group: String },
    /// Remove a notify.
    RemoveNotify { group: String, key: String },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ServerMessage {
    Reaction { group: String, key: String },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NotifyGroup {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Notify {
    /// The unique key to this notify. The key should be unique within its group.
    key: String,

    /// The group name this notify is in.
    #[serde(default)]
    group: String,

    /// Prefixes to display before body. Prefixes may be truncated or omitted if space is not enough.
    #[serde(default)]
    prefixes: Vec<String>,

    /// Body of this notify, e.g. email titles, what happened, etc.
    ///
    /// Body may be truncated if space is not enough.
    #[serde(default)]
    body: String,

    /// If true, when the user reacts on this notify, a [`ServerMessage::Rection`] will be sent to
    /// the source of this Notify.
    #[serde(default)]
    reaction: bool,

    /// If defined, this notify will automatically disappear before this time. It might disappear
    /// earlier if too many notifications are sent to this server.
    expiration: Option<Duration>,
}
