use std::sync::Arc;

use cursive::{
    event::EventResult,
    traits::Finder,
    view::{Selector, ViewWrapper},
    views::{HideableView, LinearLayout, NamedView, PaddedView, TextView},
    Vec2, View,
};
use log::info;

use crate::{model::MessageGroup, util::DirtyCheckLock};

use super::tag_view::TagView;

pub type GroupRef = Arc<DirtyCheckLock<MessageGroup>>;

pub struct GroupView {
    pub group: GroupRef,

    folded: bool,
    layout: GroupViewLayout,
    view: LinearLayout,
}

#[derive(Debug)]
struct GroupViewLayout {
    pub last_size: Vec2,
    pub size_changed: bool,
}

impl Default for GroupViewLayout {
    fn default() -> Self {
        GroupViewLayout {
            last_size: Vec2::default(),
            size_changed: true,
        }
    }
}

impl GroupView {
    pub fn new(group: GroupRef) -> Self {
        Self {
            group,
            folded: false,
            layout: Default::default(),
            view: LinearLayout::vertical(),
        }
    }

    fn name(&self) -> String {
        self.group.read(false).meta().title.clone()
    }

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

    fn available_vertical_space(&self) -> usize {
        self.layout.last_size.y.saturating_sub(1)
    }

    /// Initialize self's linear layout view
    fn init_view(&mut self) {
        assert_eq!(self.view.len(), 0, "View is changed before initialization");
        self.view.add_child(NamedView::new(
            "group-name",
            // This title will be set in dirty_check_and_update
            TextView::new(""),
        ));
        self.view.add_child(NamedView::new(
            "msgs_hide",
            HideableView::new(NamedView::new("msgs", LinearLayout::vertical())),
        ));
    }

    /// Do dirty check and update child view. This method does heavy diff checks
    /// and should only be called on relayouts.
    pub fn dirty_check_and_update(&mut self) {
        // Check folded state
        {
            let folded = self.folded;
            self.view
                .call_on_name::<HideableView<NamedView<LinearLayout>>, _, _>(
                    "msgs_hide",
                    move |b| b.set_visible(!folded),
                );
        }
        // Return if nothing has changed
        if !(self.is_dirty() || self.layout.size_changed) {
            return;
        }
        // Ensure view is initialized
        if self.view.len() != 2 {
            self.init_view()
        }
        debug_assert_eq!(self.view.len(), 2, "The view is left in an invalid state");

        // Calculate available spaces
        let max_entry_cnt = self.available_vertical_space();
        let max_pinned_cnt = (max_entry_cnt + 1) / 2;

        // Acquire read lock
        let group = self.group.read(true);

        info!(
            "group {}: dirty check: size {:?}, num_entry {}, num_pinned {}",
            group.id(),
            self.layout.last_size,
            group.msgs.len(),
            group.pinned_msgs.len()
        );

        // Set group name
        let counter = group.counter();
        let content;
        if counter > 1 {
            content = format!("- [{}] {}", counter, group.meta().title);
        } else {
            content = format!("- {}", group.meta().title);
        }
        self.view
            .call_on::<TextView, _, _>(&Selector::Name("group-name"), |v| {
                v.set_content(content);
            });

        // Set group body
        let mut body = self
            .view
            .find_name::<LinearLayout>("msgs")
            .expect("The messages view should always be present");

        //TODO: Do diff between the old body and the new message list.
        // Naive method: remove all children of that view and add more back
        let children_size = body.len();
        for i in (0..children_size).rev() {
            body.remove_child(i);
        }

        for (_id, pinned_item) in group.pinned_msgs.iter().rev().take(max_pinned_cnt) {
            body.add_child(
                LinearLayout::horizontal()
                    .child(TextView::new("P "))
                    .child(TagView::from(pinned_item)),
            );
        }

        let pinned_size = group.pinned_msgs.len();
        let remaining_size = max_entry_cnt - std::cmp::min(max_entry_cnt, pinned_size);

        for (_id, item) in group.msgs.iter().rev().take(remaining_size) {
            body.add_child(PaddedView::lrtb(2, 0, 0, 0, TagView::from(item)));
        }
    }
}

impl ViewWrapper for GroupView {
    cursive::wrap_impl!(self.view: LinearLayout);

    fn wrap_needs_relayout(&self) -> bool {
        self.is_dirty() || self.view.needs_relayout()
    }

    fn wrap_required_size(&mut self, req: Vec2) -> Vec2 {
        // take up the full space offered to self
        let grp = self.group.read(false);
        let msg_cnt = grp.msgs.len() + grp.pinned_msgs.len();
        (1, std::cmp::min(msg_cnt, req.y)).into()
    }

    fn wrap_on_event(&mut self, ch: cursive::event::Event) -> cursive::event::EventResult {
        match ch {
            cursive::event::Event::Refresh => EventResult::Consumed(None),
            _ => EventResult::Ignored,
        }
    }

    fn wrap_layout(&mut self, size: Vec2) {
        self.layout.size_changed = size != self.layout.last_size;
        self.layout.last_size = size;
        self.dirty_check_and_update();
        self.view.layout(size);
    }
}
