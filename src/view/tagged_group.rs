use std::sync::Arc;

use cursive::{Vec2, View};

use crate::{model::MessageGroup, util::DirtyCheckRwLock};

pub struct GroupView {
    pub name: String,
    pub group: Arc<DirtyCheckRwLock<MessageGroup>>,

    folded: bool,
    last_size: Vec2
}

impl GroupView {
    /// Get a reference to the group view's folded.
    pub fn folded(&self) -> bool {
        self.folded
    }

    /// Set the group view's folded status.
    pub fn set_folded(&mut self, folded: bool) {
        self.folded = folded;
    }

    fn is_dirty(&self) -> bool {
        self.group.is_dirty()
    }
}

impl View for GroupView {
    fn draw(&self, printer: &cursive::Printer) {
        todo!()
    }

    fn needs_relayout(&self) -> bool {
        self.is_dirty()
    }
}
