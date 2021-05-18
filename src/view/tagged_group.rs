use std::sync::Arc;

use cursive::{
    traits::Finder,
    view::{Selector, ViewWrapper},
    views::{HideableView, LinearLayout, NamedView, PaddedView, TextView},
    Vec2, View,
};
use smol_str::SmolStr;

use crate::{model::MessageGroup, util::DirtyCheckLock};

use super::tag_view::TagView;

pub type GroupRef = Arc<DirtyCheckLock<MessageGroup>>;

pub struct GroupView {
    pub group: GroupRef,
    pub size_limit: usize,

    folded: bool,
    view: LinearLayout,
}

impl GroupView {
    pub fn new(group: GroupRef, size_limit: usize, folded: bool) -> Self {
        Self {
            group,
            size_limit,
            folded,
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
    fn dirty_check_and_update(&mut self) {
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
        if !self.is_dirty() {
            return;
        }
        // Ensure view is initialized
        if self.view.len() != 2 {
            self.init_view()
        }

        // Acquire read lock
        let group = self.group.read(true);

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

        for (_id, pinned_item) in group.pinned_msgs.iter().rev().take(self.size_limit) {
            body.add_child(
                LinearLayout::horizontal()
                    .child(TextView::new("P "))
                    .child(TagView::from(pinned_item)),
            );
        }

        let pinned_size = group.pinned_msgs.len();
        let remaining_size = self.size_limit - std::cmp::min(self.size_limit, pinned_size);

        for (_id, item) in group.msgs.iter().rev().take(remaining_size) {
            body.add_child(PaddedView::lrtb(2, 0, 0, 0, TagView::from(item)));
        }
    }
}

impl ViewWrapper for GroupView {
    cursive::wrap_impl!(self.view: LinearLayout);

    fn wrap_needs_relayout(&self) -> bool {
        self.is_dirty() || self.with_view(|v| v.needs_relayout()).unwrap()
    }

    fn wrap_layout(&mut self, size: Vec2) {
        self.dirty_check_and_update();

        self.with_view_mut(|v| v.layout(size)).unwrap();
    }
}
