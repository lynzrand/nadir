use std::{rc::Rc, sync::Arc};

use indexmap::IndexSet;
use log::debug;
use smol_str::SmolStr;
use zi::{components::text::TextProperties, ComponentExt, FlexBasis};

use crate::{
    model::{group_list::GroupList, MessageGroup},
    util::DirtyCheckLock,
};

// use super::tag_view::TagView;

pub type GroupRef = Arc<DirtyCheckLock<MessageGroup>>;

pub struct GroupView {
    pub group: GroupRef,

    folded: bool,
}

impl GroupView {
    pub fn new(group: GroupRef) -> Self {
        Self {
            group,
            folded: false,
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

    // /// Do dirty check and update child view. This method does heavy diff checks
    // /// and should only be called on relayouts.
    // pub fn dirty_check_and_update(&mut self) {
    //     // Check folded state
    //     {
    //         let folded = self.folded;
    //         self.view
    //             .call_on_name::<HideableView<NamedView<LinearLayout>>, _, _>(
    //                 "msgs_hide",
    //                 move |b| b.set_visible(!folded),
    //             );
    //     }
    //     // Return if nothing has changed
    //     if !(self.is_dirty() || self.layout.size_changed) {
    //         return;
    //     }
    //     // Ensure view is initialized
    //     if self.view.len() != 2 {
    //         self.init_view()
    //     }
    //     debug_assert_eq!(self.view.len(), 2, "The view is left in an invalid state");

    //     // Calculate available spaces
    //     let max_entry_cnt = self.available_vertical_space();
    //     let max_pinned_cnt = (max_entry_cnt + 1) / 2;

    //     // Acquire read lock
    //     let group = self.group.read(true);

    //     // Set group name
    //     let counter = group.counter();
    //     let content;
    //     if counter > 1 {
    //         content = format!("- [{}] {}", counter, group.meta().title);
    //     } else {
    //         content = format!("- {}", group.meta().title);
    //     }
    //     self.view
    //         .call_on::<TextView, _, _>(&Selector::Name("group-name"), |v| {
    //             v.set_content(content);
    //         });

    //     // Set group body
    //     let mut body = self
    //         .view
    //         .find_name::<LinearLayout>("msgs")
    //         .expect("The messages view should always be present");

    //     let mut focus = body.get_focus_index();
    //     let focused_view = body
    //         .get_child(focus)
    //         .and_then(|x| x.downcast_ref::<TagView>());
    //     let focused_id = focused_view.map(|x| x.id.clone());

    //     debug!(
    //         "Handle focus on {}: focused: {}, id {:?}",
    //         group.meta().title,
    //         focus,
    //         focused_id
    //     );

    //     //TODO: Do diff between the old body and the new message list.
    //     // Naive method: remove all children of that view and add more back
    //     let children_size = body.len();

    //     for i in (0..children_size).rev() {
    //         body.remove_child(i);
    //     }

    //     for (_id, pinned_item) in group.pinned_msgs.iter().rev().take(max_pinned_cnt) {
    //         body.add_child(
    //             LinearLayout::horizontal()
    //                 .child(TextView::new("P "))
    //                 .child(TagView::from(pinned_item)),
    //         );
    //         if focused_id.as_ref().map_or(false, |x| x == _id) {
    //             let index = body.len() - 1;
    //             focus = index;
    //         }
    //     }

    //     let pinned_size = group.pinned_msgs.len();
    //     let remaining_size = max_entry_cnt - std::cmp::min(max_entry_cnt, pinned_size);

    //     for (_id, item) in group.msgs.iter().rev().take(remaining_size) {
    //         body.add_child(PaddedView::lrtb(2, 0, 0, 0, TagView::from(item)));
    //         if focused_id.as_ref().map_or(false, |x| x == _id) {
    //             let index = body.len() - 1;
    //             focus = index;
    //         }
    //     }

    //     // ignore the error if set index failed
    //     // if !focused {
    //     let _ = body.set_focus_index(focus);
    //     // }
    // }
}

impl zi::Component for GroupView {
    type Message = ();

    type Properties = GroupRef;

    fn create(
        properties: Self::Properties,
        frame: zi::Rect,
        link: zi::ComponentLink<Self>,
    ) -> Self {
        Self::new(properties)
    }

    fn view(&self) -> zi::Layout {
        let lock = self.group.read(true);
        zi::layout::column_iter(
            lock.pinned_msgs
                .iter()
                .chain(lock.msgs.iter())
                .map(|(k, v)| {
                    zi::components::text::Text::item_with_key(
                        FlexBasis::Auto,
                        k.as_str(),
                        TextProperties::new().content(k),
                    )
                }),
        )
    }
}
