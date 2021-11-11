use std::{
    collections::{BTreeMap, HashMap},
    iter::once,
    rc::Rc,
};

use slotmap::{DefaultKey, SlotMap};

use crate::view::group_view::GroupRef;

use super::MessageGroup;

/// A list of message groups, sorted by their metadata.
#[derive(Debug, Default)]
pub struct GroupList {
    /// groups
    groups: SlotMap<DefaultKey, GroupRef>,
    name_index: HashMap<String, DefaultKey>,
    importance_index: BTreeMap<(i32, String), DefaultKey>,
}

impl GroupList {
    /// Initialize an empty new group list
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a new group into this list, and then sort the groups to retain
    /// the order.
    ///
    /// To insert multiple groups at once, use [`add_groups`] so it gets less
    /// sorted.
    pub fn add_group(&mut self, group: GroupRef) {
        self.add_groups(once(group))
    }

    /// Insert many groups into this list
    pub fn add_groups(&mut self, groups: impl Iterator<Item = GroupRef>) {
        for g in groups {
            let read = g.read(false);
            let meta = read.meta();
            let id = meta.id.clone();
            let imp = meta.importance;
            drop(read);
            
            let group_id = self.groups.insert(g);
            self.name_index.insert(id.clone(), group_id);
            self.importance_index.insert((imp, id), group_id);
        }
    }

    pub fn remove_group(&mut self, group: impl AsRef<str>) -> Option<GroupRef> {
        // self.groups.shift_remove(group.as_ref()).map(|x| x.1)
        todo!()
    }

    pub fn remove_groups(&mut self, groups: impl Iterator<Item = impl AsRef<str>>) {
        todo!()
        // for group in groups {
        //     self.groups.swap_remove(group.as_ref());
        // }
        // self.sort_self();
    }

    pub fn get_group(&self, id: impl AsRef<str>) -> Option<&GroupRef> {
        // self.groups.get(id.as_ref()).map(|v| &v.1)
        todo!()
    }

    pub fn len(&self) -> usize {
        self.groups.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &GroupRef)> {
        self.importance_index
            .iter()
            .map(move |((_, s), key)| (s.as_str(), self.groups.get(*key).unwrap()))
        // self.groups.iter().map(|(k, (_, v))| (k.as_str(), v))
    }
}

struct GroupEntry {
    // id:
}
