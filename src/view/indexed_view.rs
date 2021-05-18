use std::{collections::HashMap, hash::Hash};

use bimap::BiHashMap;
use cursive::{views::BoxedView, View};
use slotmap::{DefaultKey, HopSlotMap};

/// A key-indexed view that is similar to ListView, but does not display their
/// keys and can use keys other than String.
pub struct IndexedView<Key> {
    children: HopSlotMap<DefaultKey, BoxedView>,
    mapping: HashMap<Key, DefaultKey>,
    sequence: Vec<DefaultKey>,
}

struct IndexedViewLayout {}

impl<Key> IndexedView<Key> where Key: 'static + Hash {}

impl<Key> View for IndexedView<Key>
where
    Key: 'static + Hash,
{
    fn draw(&self, printer: &cursive::Printer) {
        todo!()
    }
}
