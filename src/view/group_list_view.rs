use std::sync::Arc;

use cursive::{
    event::EventResult,
    traits::Nameable,
    view::ViewWrapper,
    views::{self, LinearLayout, ResizedView},
    wrap_impl, Vec2, View,
};
use log::info;

use crate::{model::group_list::GroupList, util::DirtyCheckLock};

use super::group_view::GroupView;

const MIN_SIZE_FOR_EACH_LAYOUT: usize = 3;

/// A view to dynamically reorder message groups
pub struct GroupListView {
    pub data: Arc<DirtyCheckLock<GroupList>>,
    pub if_empty: Box<dyn Fn() -> Box<dyn View>>,

    layout: GroupListViewLayout,
    view: ResizedView<LinearLayout>,
}

#[derive(Debug, Default)]
struct GroupListViewLayout {
    pub last_size: Vec2,
    pub size_changed: bool,
    pub children_sizes: Vec<usize>,
}

impl GroupListView {
    pub fn new(
        data: Arc<DirtyCheckLock<GroupList>>,
        if_empty: Box<dyn Fn() -> Box<dyn View>>,
    ) -> Self {
        GroupListView {
            data,
            if_empty,
            layout: Default::default(),
            view: ResizedView::with_full_screen(LinearLayout::vertical()),
        }
    }

    // fn is_children_dirty(&self) -> bool {
    //     self.data.is_dirty() || self.data.read(false).iter().any(|i| i.is_dirty())
    // }

    fn dirty_check_and_layout_update(&mut self) {
        let is_dirty = self.data.is_dirty();
        info!(
            "group list: dirty check: dirty {}, size {:?}",
            is_dirty, self.layout.last_size
        );
        if !(is_dirty || self.layout.size_changed) {
            return;
        }

        let guard = self.data.read(true);

        // Regenerate children views if data is dirty
        if is_dirty {
            // Note: the dirty flag only flags for group order changes
            // do data update

            // TODO: replace this naive method with a diff check
            let inner = self.view.get_inner_mut();
            let len = inner.len();

            let focus = inner.get_focus_index();
            for i in (0..len).rev() {
                inner.remove_child(i);
            }

            if guard.is_empty() {
                inner.add_child((self.if_empty)());
            } else {
                for (i, (n, v)) in guard.iter().enumerate() {
                    v.set_dirty(true);
                    inner.add_child(GroupView::new(v.clone()).with_name(format!("v-group-{}", n)));
                    inner.set_weight(i, 1);
                }
            }

            let _ = inner.set_focus_index(focus);

            log::info!("refreshed");
        }

        // Do layout updates?
    }
}

impl ViewWrapper for GroupListView {
    wrap_impl!(self.view: ResizedView<LinearLayout>);

    fn wrap_needs_relayout(&self) -> bool {
        self.view.needs_relayout() || self.data.is_dirty()
    }

    fn wrap_required_size(&mut self, req: Vec2) -> Vec2 {
        self.dirty_check_and_layout_update();
        self.view.required_size(req)
    }

    fn wrap_layout(&mut self, size: Vec2) {
        self.view.layout(size)
    }
}
