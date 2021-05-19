use std::sync::Arc;

use cursive::{view::ViewWrapper, views::LinearLayout, wrap_impl, Vec2, View};

use crate::{model::group_list::GroupList, util::DirtyCheckLock};

use super::group_view::GroupView;

const MIN_SIZE_FOR_EACH_LAYOUT: usize = 3;

/// A view to dynamically reorder message groups
pub struct GroupListView {
    pub data: Arc<DirtyCheckLock<GroupList>>,

    layout: GroupListViewLayout,
    view: LinearLayout,
}

#[derive(Debug, Default)]
struct GroupListViewLayout {
    pub last_size: Vec2,
    pub size_changed: bool,
    pub children_sizes: Vec<usize>,
}

impl GroupListView {
    fn dirty_check_and_layout_update(&mut self) {
        if !(self.data.is_dirty() || self.layout.size_changed) {
            return;
        }

        let guard = self.data.read(true);

        // Regenerate children views if data is dirty
        if self.data.is_dirty() {
            // Note: the dirty flag only flags for group order changes
            // do data update

            // TODO: replace this naive method with a diff check
            let len = self.view.len();
            for i in (0..len).rev() {
                self.view.remove_child(i);
            }

            for v in guard.iter() {
                self.view.add_child(GroupView::new(v.clone()));
            }

            self.layout.children_sizes.clear();
        }

        // Do layout updates?
    }
}

impl ViewWrapper for GroupListView {
    wrap_impl!(self.view: LinearLayout);

    fn wrap_needs_relayout(&self) -> bool {
        self.data.is_dirty()
    }

    fn wrap_layout(&mut self, size: Vec2) {
        self.layout.size_changed = size != self.layout.last_size;
        self.layout.last_size = size;
        self.dirty_check_and_layout_update();
        self.view.layout(size)
    }
}
