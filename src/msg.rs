//! Messages for intra-module interactions

use flume::Sender;
use smol_str::SmolStr;

use crate::model::{Notify, NotifyGroup};

#[derive(Debug)]
pub enum ControlMessage {
    Respond { group: SmolStr, msg: SmolStr },
}

#[derive(Debug)]
pub enum GroupMsg {
    /// Create or replace a notify group.
    PutGroup(NotifyGroup),
    /// Create or replace a notify within a group.
    PutNotify(Notify, Sender<Box<ControlMessage>>),
    /// Remove a notify group.
    RemoveGroup { group: SmolStr },
    /// Remove a notify.
    RemoveNotify { group: SmolStr, key: SmolStr },
}
