use std::sync::Arc;

use crate::{model::group_list::GroupList, util::DirtyCheckLock};
use log::debug;
use parking_lot::RwLock;
use zi::ComponentExt;
use zi::{layout, Component, FlexBasis};

use super::group_view::GroupView;

const MIN_SIZE_FOR_EACH_LAYOUT: usize = 3;

/// A view to dynamically reorder message groups
pub struct GroupListView {
    pub data: Arc<DirtyCheckLock<GroupList>>,
    // pub when_empty: Box<dyn Fn() -> zi::Layout>,
}

impl GroupListView {
    pub fn new(data: Arc<DirtyCheckLock<GroupList>>) -> Self {
        GroupListView { data }
    }
}

impl zi::Component for GroupListView {
    type Message = ();

    type Properties = (
        Arc<DirtyCheckLock<GroupList>>,
        tokio::sync::oneshot::Sender<zi::ComponentLink<Self>>,
    );

    fn create(
        properties: Self::Properties,
        frame: zi::Rect,
        link: zi::ComponentLink<Self>,
    ) -> Self {
        properties.1.send(link).unwrap();
        Self::new(properties.0)
    }

    fn view(&self) -> zi::Layout {
        let lock = self.data.read(false);
        layout::column_iter(
            lock.iter()
                .map(|(k, v)| GroupView::item_with_key(FlexBasis::Auto, k, v.clone())),
        )
    }
}

// pub enum Message {Insert()}
